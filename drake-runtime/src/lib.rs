#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use drake_types::ast::{
    Expression, ExpressionKind, KeyKind, Literal, Pattern, PatternKind, Statement, StatementKind,
};
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value<L> {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
    Array(Vec<Value<L>>),
    Table(Table<L>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<L> {
    pub value: Value<L>,
    pub defined: Range<L>,
    pub used: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table<L> {
    pub global: HashMap<String, Variable<L>>,
    pub local: HashMap<String, Variable<L>>,
}

impl<L> Default for Table<L> {
    #[inline]
    fn default() -> Self {
        Self {
            global: HashMap::new(),
            local: HashMap::new(),
        }
    }
}

impl<L> Table<L> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, global: bool, key: String, var: Variable<L>) -> Option<&Variable<L>> {
        let table = if global {
            &mut self.global
        } else {
            &mut self.local
        };

        if table.contains_key(&key) {
            Some(&table[&key])
        } else {
            table.insert(key, var);
            None
        }
    }
}

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
            match table.insert(
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
                if let Some(var) = table.insert(
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

fn key_kind(kind: KeyKind) -> Option<bool> {
    match kind {
        KeyKind::Normal => Some(true),
        KeyKind::Local => Some(false),
        KeyKind::Builtin => None,
    }
}
