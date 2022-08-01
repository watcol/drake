#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use drake_types::ast::{
    Expression, ExpressionKind, Key, KeyKind, Pattern, PatternKind, Statement, StatementKind,
    TableHeaderKind,
};
use drake_types::error::{Error, Span};
use drake_types::ir::{Builtin, Element, ElementKind, Expr, ExprKind, Ir, Table};

#[derive(Clone, Debug, PartialEq)]
struct Environment<L> {
    file_id: usize,
    root: Table<Element<L>>,
    builtin: Builtin<L>,
    current: Option<Current<L>>,
    errors: Vec<Error<L>>,
}

impl<L: Clone> Environment<L> {
    #[inline]
    fn new(file_id: usize) -> Self {
        Self {
            file_id,
            root: Table::new(),
            builtin: Builtin::new(),
            current: None,
            errors: Vec::new(),
        }
    }

    fn bind(&mut self, pattern: Pattern<L>, elem: ElementKind<L>) {
        match pattern.kind {
            PatternKind::Key(key) => {
                let table = match self.current {
                    Some(ref mut cur) => match cur.elem.as_mut_table() {
                        Some(table) => table,
                        None => {
                            self.errors.push(Error::Unexpected);
                            return;
                        }
                    },
                    None => &mut self.root,
                };

                insert_elem(table, key, elem, &mut self.errors, self.file_id);
            }
            PatternKind::Builtin(key) => self.builtin_write(key, elem),
            _ => {
                self.errors.push(Error::NotSupported {
                    feature: "unknown pattern",
                    span: Span {
                        file_id: self.file_id,
                        span: pattern.span,
                    },
                });
            }
        };
    }

    fn header(&mut self, kind: TableHeaderKind, pattern: Pattern<L>, default: Option<Expr<L>>) {
        if let Some(mut cur) = core::mem::take(&mut self.current) {
            if cur.is_movable(kind, &pattern) {
                cur.next_array(&mut self.errors);
                self.current = Some(cur);
            } else {
                self.bind(cur.pattern, into_element(cur.elem, cur.default));
                self.current = Some(Current::new(kind, pattern, default));
            }
        } else {
            self.current = Some(Current::new(kind, pattern, default));
        }
    }

    fn builtin_write(&mut self, key: Key<L>, elem: ElementKind<L>) {
        if key.kind != KeyKind::Normal {
            self.errors.push(Error::BuiltinNotFound {
                span: Span {
                    file_id: self.file_id,
                    span: key.span,
                },
            });
            return;
        }

        match key.name.as_str() {
            "output" => {
                if let Some(ref output) = self.builtin.output {
                    self.errors.push(Error::DuplicateKey {
                        existing: Some(Span {
                            file_id: self.file_id,
                            span: output.defined.clone(),
                        }),
                        found: Span {
                            file_id: self.file_id,
                            span: output.defined.clone(),
                        },
                    });
                } else {
                    self.builtin.output = Some(Element {
                        kind: elem,
                        defined: key.span,
                    });
                }
            }
            "filetype" => {
                if let Some(ref filetype) = self.builtin.filetype {
                    self.errors.push(Error::DuplicateKey {
                        existing: Some(Span {
                            file_id: self.file_id,
                            span: filetype.defined.clone(),
                        }),
                        found: Span {
                            file_id: self.file_id,
                            span: filetype.defined.clone(),
                        },
                    });
                } else {
                    self.builtin.output = Some(Element {
                        kind: elem,
                        defined: key.span,
                    });
                }
            }
            _ => self.errors.push(Error::BuiltinNotFound {
                span: Span {
                    file_id: self.file_id,
                    span: key.span,
                },
            }),
        }
    }

    fn close(mut self) -> (Ir<L>, Vec<Error<L>>) {
        if let Some(cur) = core::mem::take(&mut self.current) {
            self.bind(cur.pattern, into_element(cur.elem, cur.default));
        }

        (
            Ir {
                root: self.root,
                builtin: self.builtin,
            },
            self.errors,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Current<L> {
    pattern: Pattern<L>,
    elem: CurrentElem<L>,
    default: Option<Expr<L>>,
}

fn into_element<L>(elem: CurrentElem<L>, default: Option<Expr<L>>) -> ElementKind<L> {
    match elem {
        CurrentElem::Table(table) => ElementKind::Table(table, default),
        CurrentElem::Array(arr) => ElementKind::Array(arr, default),
    }
}

impl<L: Clone> Current<L> {
    fn new(kind: TableHeaderKind, pattern: Pattern<L>, default: Option<Expr<L>>) -> Self {
        Self {
            pattern,
            elem: match kind {
                TableHeaderKind::Normal => CurrentElem::Table(Table::new()),
                TableHeaderKind::Array => CurrentElem::Array(vec![Table::new()]),
                _ => unimplemented!(),
            },
            default,
        }
    }

    #[inline]
    fn is_movable(&self, kind: TableHeaderKind, pattern: &Pattern<L>) -> bool {
        kind == TableHeaderKind::Array
            && matches!(self.elem, CurrentElem::Array(_))
            && self.pattern == *pattern
    }

    fn next_array(&mut self, errors: &mut Vec<Error<L>>) {
        match self.elem {
            CurrentElem::Table(_) => errors.push(Error::Unexpected),
            CurrentElem::Array(ref mut arr) => arr.push(Table::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CurrentElem<L> {
    Table(Table<Element<L>>),
    Array(Vec<Table<Element<L>>>),
}

impl<L> CurrentElem<L> {
    fn as_mut_table(&mut self) -> Option<&mut Table<Element<L>>> {
        match self {
            Self::Table(table) => Some(table),
            Self::Array(arr) => arr.last_mut(),
        }
    }
}

/// Interprets an AST to IR.
pub fn interpret<L: Clone>(ast: &[Statement<L>], file_id: usize) -> (Ir<L>, Vec<Error<L>>) {
    let mut env = Environment::new(file_id);
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(ref pattern, ref expr) => {
                let expr = expression(expr.clone(), &mut env.errors, file_id);
                env.bind(pattern.clone(), ElementKind::Expr(expr))
            }
            StatementKind::TableHeader(kind, ref pattern, ref default) => {
                let default = default
                    .as_ref()
                    .map(|def| expression(def.clone(), &mut env.errors, file_id));
                env.header(kind, pattern.clone(), default)
            }
            _ => env.errors.push(Error::NotSupported {
                feature: "unknown statements",
                span: Span {
                    file_id,
                    span: stmt.span.clone(),
                },
            }),
        }
    }

    env.close()
}

fn expression<L: Clone>(
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) -> Expr<L> {
    let kind = match expr.kind {
        ExpressionKind::Literal(lit) => ExprKind::Literal(lit),
        ExpressionKind::Array(arr) => ExprKind::Array(
            arr.into_iter()
                .map(|elem| expression(elem, errors, file_id))
                .collect(),
        ),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                insert_expr(
                    &mut table,
                    key,
                    expression(expr, errors, file_id),
                    errors,
                    file_id,
                );
            }
            ExprKind::Table(table)
        }
        _ => {
            errors.push(Error::NotSupported {
                feature: "unknown expressions",
                span: Span {
                    file_id,
                    span: expr.span.clone(),
                },
            });
            ExprKind::Table(Table::new())
        }
    };

    Expr {
        kind,
        span: expr.span,
    }
}

fn insert_expr<L: Clone>(
    table: &mut Table<Expr<L>>,
    key: Key<L>,
    expr: Expr<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) {
    let table = match key.kind {
        KeyKind::Normal => &mut table.global,
        KeyKind::Local => &mut table.local,
        _ => {
            errors.push(Error::NotSupported {
                feature: "unknown keys",
                span: Span {
                    file_id,
                    span: key.span,
                },
            });
            return;
        }
    };

    if table.contains_key(&key.name) {
        errors.push(Error::DuplicateKey {
            existing: None,
            found: Span {
                file_id,
                span: key.span,
            },
        });
    } else {
        table.insert(key.name, expr);
    }
}

fn insert_elem<L: Clone>(
    table: &mut Table<Element<L>>,
    key: Key<L>,
    kind: ElementKind<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) {
    let table = match key.kind {
        KeyKind::Normal => &mut table.global,
        KeyKind::Local => &mut table.local,
        _ => {
            errors.push(Error::NotSupported {
                feature: "unknown keys",
                span: Span {
                    file_id,
                    span: key.span,
                },
            });
            return;
        }
    };

    if table.contains_key(&key.name) {
        errors.push(Error::DuplicateKey {
            existing: Some(Span {
                file_id,
                span: table[&key.name].defined.clone(),
            }),
            found: Span {
                file_id,
                span: key.span,
            },
        });
    } else {
        table.insert(
            key.name,
            Element {
                kind,
                defined: key.span,
            },
        );
    }
}
