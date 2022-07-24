#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, KeyKind, Literal, Pattern, PatternKind, Statement, StatementKind,
    TableHeaderKind,
};
use drake_types::runtime::{Element, Table, Value};

/// Errors for runtimes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<L> {
    DuplicateKey {
        existing: Range<L>,
        found: Range<L>,
    },
    KindMismatch {
        expect: Vec<Kind>,
        found: Kind,
        span: Range<L>,
    },
    UnallowedDefaultValue {
        span: Range<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Range<L>,
    },
    Unexpected,
}

/// Name of value kinds for errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Character,
    String,
    Integer,
    Float,
    Array,
    Table,
}

impl Kind {
    pub fn from_value<L>(val: &Value<L>) -> Self {
        match val {
            Value::Character(_) => Self::Character,
            Value::String(_) => Self::String,
            Value::Integer(_) => Self::Integer,
            Value::Float(_) => Self::Float,
            Value::Array(_) => Self::Array,
            Value::Table(_) => Self::Table,
        }
    }
}

struct Cursor<L> {
    kind: CursorKind<L>,
    global: bool,
    key: String,
    origin: Option<(Box<Cursor<L>>, Range<L>)>,
}

impl<L: Clone> Cursor<L> {
    fn as_mut_table(&mut self) -> Option<&mut Table<L>> {
        match self.kind {
            CursorKind::Table(ref mut table) => Some(table),
            CursorKind::Array(ref mut tables) => match tables.last_mut() {
                Some(Value::Table(ref mut table)) => Some(table),
                _ => None,
            },
        }
    }

    fn new_root() -> Self {
        Self {
            kind: CursorKind::Table(Table::new()),
            global: true,
            key: String::from("<root>"),
            origin: None,
        }
    }

    fn new_table(
        self: Box<Self>,
        kind: TableHeaderKind,
        global: bool,
        key: String,
        span: Range<L>,
        default: Option<(Value<L>, Range<L>)>,
        errors: &mut Vec<Error<L>>,
    ) -> Self {
        match kind {
            TableHeaderKind::Normal => {
                let table = match default {
                    Some((Value::Table(table), _)) => table,
                    Some((val, default_span)) => {
                        errors.push(Error::KindMismatch {
                            expect: vec![Kind::Table],
                            found: Kind::from_value(&val),
                            span: default_span,
                        });
                        Table::new()
                    }
                    None => Table::new(),
                };
                Self {
                    kind: CursorKind::Table(table),
                    global,
                    key,
                    origin: Some((self.bind(errors), span)),
                }
            }
            TableHeaderKind::Array if global == self.global && key == self.key => match self.kind {
                CursorKind::Table(_) => self
                    .bind(errors)
                    .new_table(kind, global, key, span, default, errors),
                CursorKind::Array(mut arr) => {
                    if let Some((_, default_span)) = default {
                        errors.push(Error::UnallowedDefaultValue { span: default_span });
                    }

                    arr.push(Value::Table(Table::new()));

                    Self {
                        kind: CursorKind::Array(arr),
                        global,
                        key,
                        origin: self.origin,
                    }
                }
            },
            TableHeaderKind::Array => {
                let mut arr = match default {
                    Some((Value::Array(arr), _)) => arr,
                    Some((val, default_span)) => {
                        errors.push(Error::KindMismatch {
                            expect: vec![Kind::Array],
                            found: Kind::from_value(&val),
                            span: default_span,
                        });
                        Vec::new()
                    }
                    None => Vec::new(),
                };

                arr.push(Value::Table(Table::new()));

                Self {
                    kind: CursorKind::Array(arr),
                    global,
                    key,
                    origin: Some((self.bind(errors), span)),
                }
            }
        }
    }

    fn bind(self: Box<Self>, errors: &mut Vec<Error<L>>) -> Box<Self> {
        match self.origin {
            Some((mut new, span)) => {
                match new.as_mut_table() {
                    Some(t) => {
                        if let Some(var) = table_insert(
                            t,
                            self.global,
                            self.key,
                            Element {
                                value: self.kind.into_value(),
                                defined: span.clone(),
                                used: self.global,
                            },
                        ) {
                            errors.push(Error::DuplicateKey {
                                existing: var.defined.clone(),
                                found: span,
                            })
                        }
                    }
                    None => errors.push(Error::Unexpected),
                }
                new
            }
            None => self,
        }
    }
}

enum CursorKind<L> {
    Table(Table<L>),
    Array(Vec<Value<L>>),
}

impl<L> CursorKind<L> {
    fn into_value(self) -> Value<L> {
        match self {
            Self::Table(table) => Value::Table(table),
            Self::Array(arr) => Value::Array(arr),
        }
    }
}

/// Evaluates an AST to a value.
pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>, errors: &mut Vec<Error<L>>) -> Value<L> {
    let mut cursor = Box::new(Cursor::new_root());
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(pat, expr) => match cursor.as_mut_table() {
                Some(t) => bind(t, pat, expr, errors),
                None => errors.push(Error::Unexpected),
            },
            StatementKind::TableHeader(kind, pat, default) => {
                header(&mut cursor, kind, pat, default, errors)
            }
        }
    }

    cursor.bind(errors).kind.into_value()
}

fn header<L: Clone>(
    cursor: &mut Box<Cursor<L>>,
    kind: TableHeaderKind,
    pat: Pattern<L>,
    default: Option<Expression<L>>,
    errors: &mut Vec<Error<L>>,
) {
    let (global, key, span) = match pat.kind {
        PatternKind::Key(key) => match key_kind(key.kind) {
            Some(b) => (b, key.name, key.span),
            None => {
                errors.push(Error::NotSupported {
                    feature: "built-in keys",
                    span: key.span,
                });
                return;
            }
        },
    };

    let default = default.map(|expr| {
        let span = expr.span.clone();
        (expr_to_value(expr, errors), span)
    });

    unsafe {
        use core::ptr;
        let cur = ptr::read(cursor);
        ptr::write(
            cursor,
            Box::new(cur.new_table(kind, global, key, span, default, errors)),
        )
    }
}

fn bind<L: Clone>(
    table: &mut Table<L>,
    pat: Pattern<L>,
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
) {
    match pat.kind {
        PatternKind::Key(key) => {
            let global = match key_kind(key.kind) {
                Some(b) => b,
                None => {
                    errors.push(Error::NotSupported {
                        feature: "built-in keys",
                        span: key.span,
                    });
                    return;
                }
            };

            if let Some(var) = table_insert(
                table,
                global,
                key.name,
                Element {
                    value: expr_to_value(expr, errors),
                    defined: key.span.clone(),
                    used: global,
                },
            ) {
                errors.push(Error::DuplicateKey {
                    existing: var.defined.clone(),
                    found: key.span,
                })
            }
        }
    }
}

fn expr_to_value<L: Clone>(expr: Expression<L>, errors: &mut Vec<Error<L>>) -> Value<L> {
    match expr.kind {
        ExpressionKind::Literal(Literal::Character(c)) => Value::Character(c),
        ExpressionKind::Literal(Literal::String(s)) => Value::String(s),
        ExpressionKind::Literal(Literal::Integer(i)) => Value::Integer(i),
        ExpressionKind::Literal(Literal::Float(f)) => Value::Float(f),
        ExpressionKind::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|elem| expr_to_value(elem, errors))
                .collect(),
        ),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                let global = match key_kind(key.kind) {
                    Some(b) => b,
                    None => {
                        errors.push(Error::NotSupported {
                            feature: "built-in keys",
                            span: key.span,
                        });
                        continue;
                    }
                };
                if let Some(var) = table_insert(
                    &mut table,
                    global,
                    key.name,
                    Element {
                        value: expr_to_value(expr, errors),
                        defined: key.span.clone(),
                        used: global,
                    },
                ) {
                    errors.push(Error::DuplicateKey {
                        existing: var.defined.clone(),
                        found: key.span,
                    });
                }
            }
            Value::Table(table)
        }
    }
}

fn table_insert<L>(
    table: &mut Table<L>,
    global: bool,
    key: String,
    elem: Element<L>,
) -> Option<&Element<L>> {
    let table = if global {
        &mut table.global
    } else {
        &mut table.local
    };

    if table.contains_key(&key) {
        Some(&table[&key])
    } else {
        table.insert(key, elem);
        None
    }
}

fn key_kind(kind: KeyKind) -> Option<bool> {
    match kind {
        KeyKind::Normal => Some(true),
        KeyKind::Local => Some(false),
        KeyKind::Builtin => None,
    }
}
