#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, Key, KeyKind, Literal, Pattern, PatternKind, Statement,
    StatementKind, TableHeaderKind,
};
use drake_types::error::{Error, Kind, Span};
use drake_types::runtime::{Builtin, Element, Snapshot, Table, Value};

#[derive(Clone, Debug, PartialEq)]
struct Environment<L> {
    file_id: usize,
    root: Table<L>,
    builtin: Builtin,
    current: Option<Current<L>>,
    errors: Vec<Error<L>>,
}

impl<L: Clone> Environment<L> {
    #[inline]
    fn new(file_id: usize) -> Self {
        Self {
            file_id,
            root: Table::new(),
            builtin: Builtin::default(),
            current: None,
            errors: Vec::new(),
        }
    }

    fn bind(&mut self, pattern: Pattern<L>, value: Value<L>) {
        match pattern.kind {
            PatternKind::Key(key) => {
                let table = match self.current {
                    Some(ref mut cur) => match cur.value.as_mut_table() {
                        Some(table) => table,
                        None => {
                            self.errors.push(Error::Unexpected);
                            return;
                        }
                    },
                    None => &mut self.root,
                };

                insert(table, key, value, &mut self.errors, self.file_id);
            }
            PatternKind::Builtin(key) => self.builtin_write(key, value),
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

    fn header(
        &mut self,
        kind: TableHeaderKind,
        pattern: Pattern<L>,
        default: Option<Expression<L>>,
    ) {
        let (mut default, default_span) = default
            .map(|expr| {
                let (val, span) = expr_to_value(expr, &mut self.errors, self.file_id);
                (
                    assert_table(val, span.clone(), &mut self.errors, self.file_id)
                        .unwrap_or_default(),
                    Some(span),
                )
            })
            .unwrap_or_default();
        default.make_default();

        if let Some(mut cur) = core::mem::take(&mut self.current) {
            if cur.is_movable(kind, &pattern) {
                if let Some(span) = default_span {
                    self.errors.push(Error::UnallowedDefaultValue {
                        span: Span {
                            file_id: self.file_id,
                            span,
                        },
                    });
                }

                cur.next_array(&mut self.errors);
                self.current = Some(cur);
            } else {
                self.bind(cur.pattern, cur.value.into_value());
                self.current = Some(Current::new(kind, pattern, default));
            }
        } else {
            self.current = Some(Current::new(kind, pattern, default));
        }
    }

    fn builtin_write(&mut self, key: Key<L>, value: Value<L>) {
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
                if let Value::String(s) = value {
                    self.builtin.output = Some(s);
                } else {
                    self.errors.push(Error::KindMismatch {
                        expect: vec![Kind::String],
                        found: value.kind(),
                        span: Span {
                            file_id: self.file_id,
                            span: key.span,
                        },
                    })
                }
            }
            "filetype" => {
                if let Value::String(s) = value {
                    self.builtin.filetype = Some(s);
                } else {
                    self.errors.push(Error::KindMismatch {
                        expect: vec![Kind::String],
                        found: value.kind(),
                        span: Span {
                            file_id: self.file_id,
                            span: key.span,
                        },
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

    fn close(mut self) -> Snapshot<L> {
        if let Some(cur) = core::mem::take(&mut self.current) {
            self.bind(cur.pattern, cur.value.into_value());
        }

        Snapshot {
            root: self.root,
            builtin: self.builtin,
            errors: self.errors,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Current<L> {
    pattern: Pattern<L>,
    value: CurrentValue<L>,
}

impl<L: Clone> Current<L> {
    fn new(kind: TableHeaderKind, pattern: Pattern<L>, default: Table<L>) -> Self {
        Self {
            pattern,
            value: match kind {
                TableHeaderKind::Normal => CurrentValue::Table(default),
                TableHeaderKind::Array => CurrentValue::Array(vec![default.clone()], default),
                _ => unimplemented!(),
            },
        }
    }

    #[inline]
    fn is_movable(&self, kind: TableHeaderKind, pattern: &Pattern<L>) -> bool {
        kind == TableHeaderKind::Array
            && matches!(self.value, CurrentValue::Array(_, _))
            && self.pattern == *pattern
    }

    fn next_array(&mut self, errors: &mut Vec<Error<L>>) {
        match self.value {
            CurrentValue::Table(_) => errors.push(Error::Unexpected),
            CurrentValue::Array(ref mut arr, ref default) => arr.push(default.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CurrentValue<L> {
    Table(Table<L>),
    Array(Vec<Table<L>>, Table<L>),
}

impl<L> CurrentValue<L> {
    fn as_mut_table(&mut self) -> Option<&mut Table<L>> {
        match self {
            Self::Table(table) => Some(table),
            Self::Array(arr, _) => arr.last_mut(),
        }
    }

    fn into_value(self) -> Value<L> {
        match self {
            Self::Table(table) => Value::Table(table),
            Self::Array(arr, _) => Value::Array(arr.into_iter().map(Value::Table).collect()),
        }
    }
}

/// Evaluates an AST to a value.
pub fn evaluate<L: Clone>(ast: &[Statement<L>], file_id: usize) -> Snapshot<L> {
    let mut env = Environment::new(file_id);
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(ref pattern, ref expr) => {
                let value = expr_to_value(expr.clone(), &mut env.errors, file_id).0;
                env.bind(pattern.clone(), value)
            }
            StatementKind::TableHeader(kind, ref pattern, ref default) => {
                env.header(kind, pattern.clone(), default.clone())
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

fn expr_to_value<L: Clone>(
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) -> (Value<L>, Range<L>) {
    let val = match expr.kind {
        ExpressionKind::Literal(Literal::Character(c)) => Value::Character(c),
        ExpressionKind::Literal(Literal::String(s)) => Value::String(s),
        ExpressionKind::Literal(Literal::Integer(i)) => Value::Integer(i),
        ExpressionKind::Literal(Literal::Float(f)) => Value::Float(f),
        ExpressionKind::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|elem| expr_to_value(elem, errors, file_id).0)
                .collect(),
        ),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                insert(
                    &mut table,
                    key,
                    expr_to_value(expr, errors, file_id).0,
                    errors,
                    file_id,
                );
            }
            Value::Table(table)
        }
        _ => {
            errors.push(Error::NotSupported {
                feature: "unknown expressions",
                span: Span {
                    file_id,
                    span: expr.span.clone(),
                },
            });
            Value::Table(Table::new())
        }
    };

    (val, expr.span)
}

fn assert_table<L>(
    val: Value<L>,
    span: Range<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) -> Option<Table<L>> {
    match val {
        Value::Table(table) => Some(table),
        val => {
            errors.push(Error::KindMismatch {
                expect: vec![Kind::Table],
                found: val.kind(),
                span: Span { file_id, span },
            });
            None
        }
    }
}

fn insert<L: Clone>(
    table: &mut Table<L>,
    key: Key<L>,
    value: Value<L>,
    errors: &mut Vec<Error<L>>,
    file_id: usize,
) {
    let (table, used) = match key.kind {
        KeyKind::Normal => (&mut table.global, true),
        KeyKind::Local => (&mut table.global, false),
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

    if table.contains_key(&key.name) && !table[&key.name].default {
        errors.push(Error::DuplicateKey {
            existing: Span {
                file_id,
                span: table[&key.name].defined.clone(),
            },
            found: Span {
                file_id,
                span: key.span,
            },
        });
    } else {
        table.insert(
            key.name,
            Element {
                value,
                defined: key.span,
                default: false,
                used,
            },
        );
    }
}
