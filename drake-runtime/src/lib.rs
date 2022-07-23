#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, KeyKind, Literal, Pattern, PatternKind, Statement, StatementKind,
};
use drake_types::runtime::{Element, Table, Value};

/// Errors for runtimes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<L> {
    DuplicateKey {
        existing: Range<L>,
        found: Range<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Range<L>,
    },
}

/// Evaluates an AST to a value.
pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>, errors: &mut Vec<Error<L>>) -> Value<L> {
    let mut root = Table::new();
    let current_table = &mut root;
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(pat, expr) => bind(current_table, pat, expr, errors),
            StatementKind::TableHeader(_, _, _) => unimplemented!(),
        }
    }
    Value::Table(root)
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
