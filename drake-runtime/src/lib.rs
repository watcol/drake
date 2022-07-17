#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, Key, KeyKind, Literal, Pattern, PatternKind, Statement,
    StatementKind,
};
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),
}

pub struct Error<L> {
    pub msg: &'static str,
    pub span: Range<L>,
}

pub fn evaluate<L: Clone>(ast: Vec<Statement<L>>) -> Result<Value, Error<L>> {
    let mut root = HashMap::with_capacity((ast.len() / 5) * 4);
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
    table: &mut HashMap<String, Value>,
    pat: Pattern<L>,
    expr: Expression<L>,
) -> Result<(), Error<L>> {
    match pat.kind {
        PatternKind::Key(key) => {
            let span = key.span.clone();
            if let Some(name) = key_to_str(key)? {
                if table.contains_key(&name) {
                    Err(Error {
                        msg: "This key is already used.",
                        span,
                    })
                } else {
                    table.insert(name, expr_to_value(expr)?);
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
    }
}

fn expr_to_value<L: Clone>(expr: Expression<L>) -> Result<Value, Error<L>> {
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
            let mut table = HashMap::with_capacity(arr.len());
            for (key, expr) in arr {
                let span = key.span.clone();
                if let Some(name) = key_to_str(key)? {
                    if table.contains_key(&name) {
                        return Err(Error {
                            msg: "This key is already used.",
                            span,
                        });
                    }
                    table.insert(name, expr_to_value(expr)?);
                }
            }
            Ok(Value::Table(table))
        }
    }
}

fn key_to_str<L>(key: Key<L>) -> Result<Option<String>, Error<L>> {
    match key.kind {
        KeyKind::Normal => Ok(Some(key.name)),
        KeyKind::Local => Ok(None),
        KeyKind::Builtin => Err(Error {
            msg: "Built-in keys are not supported yet.",
            span: key.span,
        }),
    }
}
