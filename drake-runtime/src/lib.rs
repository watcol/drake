#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, Literal, Pattern, PatternKind, Statement, StatementKind,
    TableHeaderKind,
};
use drake_types::runtime::{Error, Kind, Snapshot, Table, Value};

#[derive(Clone, Debug, PartialEq)]
struct Environment<L> {
    root: Table<L>,
    current: Option<Current<L>>,
    errors: Vec<Error<L>>,
}

impl<L> Default for Environment<L> {
    #[inline]
    fn default() -> Self {
        Self {
            root: Table::new(),
            current: None,
            errors: Vec::new(),
        }
    }
}

impl<L: Clone> Environment<L> {
    fn bind(&mut self, pattern: Pattern<L>, value: Value<L>) {
        let (table, key) = match pattern.kind {
            PatternKind::Key(key) => (
                match self.current {
                    Some(ref mut cur) => match cur.value.as_mut_table() {
                        Some(table) => table,
                        None => {
                            self.errors.push(Error::Unexpected);
                            return;
                        }
                    },
                    None => &mut self.root,
                },
                key,
            ),
            _ => {
                self.errors.push(Error::NotSupported {
                    feature: "unknown pattern",
                    span: pattern.span,
                });
                return;
            }
        };

        if let Err(err) = table.insert(key, value) {
            self.errors.push(err);
        }
    }

    fn header(&mut self, kind: TableHeaderKind, pattern: Pattern<L>, mut default: Table<L>) {
        default.make_default();
        if let Some(mut cur) = core::mem::take(&mut self.current) {
            if cur.is_movable(kind, &pattern) {
                cur.next_array(default, &mut self.errors);
                self.current = Some(cur);
            } else {
                self.bind(cur.pattern, cur.value.into_value());
                self.current = Some(Current::new(kind, pattern, default));
            }
        } else {
            self.current = Some(Current::new(kind, pattern, default));
        }
    }

    fn close(mut self) -> Snapshot<L> {
        if let Some(cur) = core::mem::take(&mut self.current) {
            self.bind(cur.pattern, cur.value.into_value());
        }

        Snapshot {
            root: self.root,
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
                TableHeaderKind::Array => CurrentValue::Array(vec![default]),
                _ => unimplemented!(),
            },
        }
    }

    #[inline]
    fn is_movable(&self, kind: TableHeaderKind, pattern: &Pattern<L>) -> bool {
        kind == TableHeaderKind::Array
            && matches!(self.value, CurrentValue::Array(_))
            && self.pattern == *pattern
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
pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>) -> Snapshot<L> {
    let mut env = Environment::default();
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(pattern, expr) => {
                let value = expr_to_value(expr, &mut env.errors).0;
                env.bind(pattern, value)
            }
            StatementKind::TableHeader(kind, pattern, default) => {
                let default = default
                    .and_then(|expr| {
                        let (val, span) = expr_to_value(expr, &mut env.errors);
                        assert_table(val, span, &mut env.errors)
                    })
                    .unwrap_or_default();
                env.header(kind, pattern, default)
            }
            _ => env.errors.push(Error::NotSupported {
                feature: "unknown statements",
                span: stmt.span,
            }),
        }
    }

    env.close()
}

fn expr_to_value<L: Clone>(
    expr: Expression<L>,
    errors: &mut Vec<Error<L>>,
) -> (Value<L>, Range<L>) {
    let val = match expr.kind {
        ExpressionKind::Literal(Literal::Character(c)) => Value::Character(c),
        ExpressionKind::Literal(Literal::String(s)) => Value::String(s),
        ExpressionKind::Literal(Literal::Integer(i)) => Value::Integer(i),
        ExpressionKind::Literal(Literal::Float(f)) => Value::Float(f),
        ExpressionKind::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|elem| expr_to_value(elem, errors).0)
                .collect(),
        ),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                if let Err(err) = table.insert(key, expr_to_value(expr, errors).0) {
                    errors.push(err);
                }
            }
            Value::Table(table)
        }
        _ => {
            errors.push(Error::NotSupported {
                feature: "unknown expressions",
                span: expr.span.clone(),
            });
            Value::Table(Table::new())
        }
    };

    (val, expr.span)
}

fn assert_table<L: Clone>(
    val: Value<L>,
    span: Range<L>,
    errors: &mut Vec<Error<L>>,
) -> Option<Table<L>> {
    match val {
        Value::Table(table) => Some(table),
        val => {
            errors.push(Error::KindMismatch {
                expect: vec![Kind::Table],
                found: val.kind(),
                span,
            });
            None
        }
    }
}
