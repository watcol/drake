#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use drake_types::ast::{
    Expression, ExpressionKind, KeyKind, Literal, Pattern, PatternKind, Statement, StatementKind,
};
use drake_types::runtime::{Error, Table, Value, Variable};

pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>) -> Result<Value<L>, Error<L>> {
    let mut root = Table::new();
    let current_table = &mut root;
    for stmt in ast {
        match stmt.kind {
            StatementKind::ValueBinding(pat, expr) => bind(current_table, pat, expr)?,
            StatementKind::TableHeader(_, _, _) => unimplemented!(),
        }
    }
    Ok(Value::Table(root))
}

fn bind<L: Clone>(
    table: &mut Table<L>,
    pat: Pattern<L>,
    expr: Expression<L>,
) -> Result<(), Error<L>> {
    match pat.kind {
        PatternKind::Key(key) => {
            let global = match key_kind(key.kind) {
                Some(b) => b,
                None => {
                    return Err(Error::NotSupported {
                        feature: "built-in keys",
                        span: key.span,
                    })
                }
            };
            match table_insert(
                table,
                global,
                key.name,
                Variable {
                    value: expr_to_value(expr)?,
                    defined: key.span.clone(),
                    used: global,
                },
            ) {
                Some(var) => Err(Error::DuplicateKey {
                    existing: var.defined.clone(),
                    found: key.span,
                }),
                None => Ok(()),
            }
        }
    }
}

fn expr_to_value<L: Clone>(expr: Expression<L>) -> Result<Value<L>, Error<L>> {
    match expr.kind {
        ExpressionKind::Literal(Literal::Character(c)) => Ok(Value::Character(c)),
        ExpressionKind::Literal(Literal::String(s)) => Ok(Value::String(s)),
        ExpressionKind::Literal(Literal::Integer(i)) => Ok(Value::Integer(i)),
        ExpressionKind::Literal(Literal::Float(f)) => Ok(Value::Float(f)),
        ExpressionKind::Array(arr) => Ok(Value::Array(
            arr.into_iter()
                .map(expr_to_value)
                .collect::<Result<Vec<_>, _>>()?,
        )),
        ExpressionKind::InlineTable(arr) => {
            let mut table = Table::new();
            for (key, expr) in arr {
                let global = match key_kind(key.kind) {
                    Some(b) => b,
                    None => {
                        return Err(Error::NotSupported {
                            feature: "built-in keys",
                            span: key.span,
                        })
                    }
                };
                if let Some(var) = table_insert(
                    &mut table,
                    global,
                    key.name,
                    Variable {
                        value: expr_to_value(expr)?,
                        defined: key.span.clone(),
                        used: global,
                    },
                ) {
                    return Err(Error::DuplicateKey {
                        existing: var.defined.clone(),
                        found: key.span,
                    });
                }
            }
            Ok(Value::Table(table))
        }
    }
}

fn table_insert<L>(
    table: &mut Table<L>,
    global: bool,
    key: String,
    var: Variable<L>,
) -> Option<&Variable<L>> {
    let table = if global {
        &mut table.global
    } else {
        &mut table.local
    };

    if table.contains_key(&key) {
        Some(&table[&key])
    } else {
        table.insert(key, var);
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
