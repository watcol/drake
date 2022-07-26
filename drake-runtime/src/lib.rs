#![no_std]
extern crate alloc;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<L> {
    root: Table<L>,
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

#[derive(Clone, Debug, PartialEq)]
struct Environment<L> {
    root: Table<L>,
    current: Option<Current<L>>,
}

impl<L> Default for Environment<L> {
    #[inline]
    fn default() -> Self {
        Self {
            root: Table::new(),
            current: None,
        }
    }
}

impl<L: Clone> Environment<L> {
    fn bind(&mut self, pattern: Pattern<L>, value: Value<L>, errors: &mut Vec<Error<L>>) {
        let (table, key) = match pattern.kind {
            PatternKind::Key(key) => (
                match self.current {
                    Some(ref mut cur) => match cur.value.as_mut_table() {
                        Some(table) => table,
                        None => {
                            errors.push(Error::Unexpected);
                            return;
                        }
                    },
                    None => &mut self.root,
                },
                key,
            ),
        };

        table_insert(table, key, value, errors);
    }

    fn header(
        &mut self,
        kind: TableHeaderKind,
        pattern: Pattern<L>,
        mut default: Table<L>,
        errors: &mut Vec<Error<L>>,
    ) {
        default.make_default();
        if let Some(mut cur) = core::mem::take(&mut self.current) {
            if cur.is_movable(kind, &pattern) {
                cur.next_array(default, errors);
                self.current = Some(cur);
            } else {
                self.bind(cur.pattern, cur.value.into_value(), errors);
                self.current = Some(Current::new(kind, pattern, default));
            }
        } else {
            self.current = Some(Current::new(kind, pattern, default));
        }
    }

    fn close(mut self, errors: &mut Vec<Error<L>>) -> Snapshot<L> {
        if let Some(cur) = core::mem::take(&mut self.current) {
            self.bind(cur.pattern, cur.value.into_value(), errors);
        }

        Snapshot { root: self.root }
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
                TableHeaderKind::Array => CurrentValue::Array(vec![default]),
            },
        }
    }

    #[inline]
    fn is_movable(&self, kind: TableHeaderKind, pattern: &Pattern<L>) -> bool {
        kind == TableHeaderKind::Array
            && matches!(self.value, CurrentValue::Array(_))
            && match (&self.pattern.kind, &pattern.kind) {
                (PatternKind::Key(key1), PatternKind::Key(key2)) => {
                    key1.kind == key2.kind && key1.name == key2.name
                }
            }
    }

    fn next_array(&mut self, default: Table<L>, errors: &mut Vec<Error<L>>) {
        match self.value {
            CurrentValue::Table(_) => errors.push(Error::Unexpected),
            CurrentValue::Array(ref mut arr) => arr.push(default),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CurrentValue<L> {
    Table(Table<L>),
    Array(Vec<Table<L>>),
}

impl<L> CurrentValue<L> {
    fn as_mut_table(&mut self) -> Option<&mut Table<L>> {
        match self {
            Self::Table(table) => Some(table),
            Self::Array(arr) => arr.last_mut(),
        }
    }

    fn into_value(self) -> Value<L> {
        match self {
            Self::Table(table) => Value::Table(table),
            Self::Array(arr) => Value::Array(arr.into_iter().map(Value::Table).collect()),
        }
    }
}

/// Evaluates an AST to a value.
pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>, errors: &mut Vec<Error<L>>) -> Snapshot<L> {
    let mut env = Environment::default();
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(pattern, expr) => {
                env.bind(pattern, expr_to_value(expr.kind, errors), errors)
            }
            StatementKind::TableHeader(kind, pattern, default) => env.header(
                kind,
                pattern,
                default
                    .and_then(|expr| expr_to_table(expr, errors))
                    .unwrap_or_default(),
                errors,
            ),
        }
    }

    env.close(errors)
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

    if table.contains_key(&name) && !table[&name].default {
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
                default: false,
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
