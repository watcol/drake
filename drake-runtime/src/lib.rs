#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, Key, KeyKind, Literal, Pattern, PatternKind, Statement,
    StatementKind, TableHeaderKind,
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
    origin: Option<(Box<Cursor<L>>, Key<L>)>,
}

impl<L: Clone> Cursor<L> {
    fn as_mut_table(&mut self, errors: &mut Vec<Error<L>>) -> Option<&mut Table<L>> {
        match self.kind {
            CursorKind::Table(ref mut table) => Some(table),
            CursorKind::Array(ref mut tables) => match tables.last_mut() {
                Some(Value::Table(ref mut table)) => Some(table),
                _ => {
                    errors.push(Error::Unexpected);
                    None
                }
            },
        }
    }

    fn new_root() -> Self {
        Self {
            kind: CursorKind::Table(Table::new()),
            origin: None,
        }
    }

    fn new_table(
        self: Box<Self>,
        kind: TableHeaderKind,
        key: Key<L>,
        default: Option<Expression<L>>,
        errors: &mut Vec<Error<L>>,
    ) -> Self {
        match kind {
            TableHeaderKind::Normal => {
                let table = match default {
                    Some(expr) => expr_to_table(expr, errors).unwrap_or_default(),
                    None => Table::new(),
                };
                Self {
                    kind: CursorKind::Table(table),
                    origin: Some((self.bind(errors), key)),
                }
            }
            TableHeaderKind::Array
                if self
                    .origin
                    .as_ref()
                    .map(|(_, org_key)| key.kind == org_key.kind && key.name == key.name)
                    .unwrap_or_default() =>
            {
                match self.kind {
                    CursorKind::Table(_) => self.bind(errors).new_table(kind, key, default, errors),
                    CursorKind::Array(mut arr) => {
                        if let Some(expr) = default {
                            errors.push(Error::UnallowedDefaultValue { span: expr.span });
                        }

                        arr.push(Value::Table(Table::new()));

                        Self {
                            kind: CursorKind::Array(arr),
                            origin: self.origin,
                        }
                    }
                }
            }
            TableHeaderKind::Array => {
                let mut arr = match default {
                    Some(expr) => expr_to_array(expr, errors).unwrap_or_default(),
                    None => Vec::new(),
                };

                arr.push(Value::Table(Table::new()));

                Self {
                    kind: CursorKind::Array(arr),
                    origin: Some((self.bind(errors), key)),
                }
            }
        }
    }

    fn bind(self: Box<Self>, errors: &mut Vec<Error<L>>) -> Box<Self> {
        match self.origin {
            Some((mut new, key)) => {
                if let Some(t) = new.as_mut_table(errors) {
                    table_insert(t, key, self.kind.into_value(), errors);
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
            StatementKind::ValueBinding(pat, expr) => match cursor.as_mut_table(errors) {
                Some(t) => bind(t, pat, expr, errors),
                None => continue,
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
    match pat.kind {
        PatternKind::Key(key) => unsafe {
            use core::ptr;
            let cur = ptr::read(cursor);
            ptr::write(cursor, Box::new(cur.new_table(kind, key, default, errors)))
        },
    }
}

fn bind<L: Clone>(
    table: &mut Table<L>,
    pat: Pattern<L>,
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
) {
    let val = expr_to_value(expr.kind, errors);
    match pat.kind {
        PatternKind::Key(key) => table_insert(table, key, val, errors),
    }
}

fn expr_to_value<L: Clone>(expr: ExpressionKind<L>, errors: &mut Vec<Error<L>>) -> Value<L> {
    match expr {
        ExpressionKind::Literal(Literal::Character(c)) => Value::Character(c),
        ExpressionKind::Literal(Literal::String(s)) => Value::String(s),
        ExpressionKind::Literal(Literal::Integer(i)) => Value::Integer(i),
        ExpressionKind::Literal(Literal::Float(f)) => Value::Float(f),
        ExpressionKind::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|elem| expr_to_value(elem.kind, errors))
                .collect(),
        ),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                table_insert(&mut table, key, expr_to_value(expr.kind, errors), errors);
            }
            Value::Table(table)
        }
    }
}

fn expr_to_table<L: Clone>(expr: Expression<L>, errors: &mut Vec<Error<L>>) -> Option<Table<L>> {
    match expr_to_value(expr.kind, errors) {
        Value::Table(table) => Some(table),
        val => {
            errors.push(Error::KindMismatch {
                expect: vec![Kind::Table],
                found: Kind::from_value(&val),
                span: expr.span,
            });
            None
        }
    }
}

fn expr_to_array<L: Clone>(
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
) -> Option<Vec<Value<L>>> {
    match expr_to_value(expr.kind, errors) {
        Value::Array(arr) => Some(arr),
        val => {
            errors.push(Error::KindMismatch {
                expect: vec![Kind::Array],
                found: Kind::from_value(&val),
                span: expr.span,
            });
            None
        }
    }
}

fn table_insert<L: Clone>(
    table: &mut Table<L>,
    key: Key<L>,
    value: Value<L>,
    errors: &mut Vec<Error<L>>,
) {
    let (global, name, span) = match key_destruct(key, errors) {
        Some(key) => key,
        None => return,
    };

    let table = if global {
        &mut table.global
    } else {
        &mut table.local
    };

    if table.contains_key(&name) {
        errors.push(Error::DuplicateKey {
            existing: table[&name].defined.clone(),
            found: span,
        });
    } else {
        table.insert(
            name,
            Element {
                value,
                defined: span,
                used: global,
            },
        );
    }
}

fn key_destruct<L>(key: Key<L>, errors: &mut Vec<Error<L>>) -> Option<(bool, String, Range<L>)> {
    match key.kind {
        KeyKind::Normal => Some((true, key.name, key.span)),
        KeyKind::Local => Some((false, key.name, key.span)),
        KeyKind::Builtin => {
            errors.push(Error::NotSupported {
                feature: "built-in keys",
                span: key.span,
            });
            None
        }
    }
}
