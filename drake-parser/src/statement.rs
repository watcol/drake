//! Parsers for statements
#[cfg(test)]
mod tests;

use drake_types::ast::{Expression, Pattern, Statement, StatementKind, TableHeaderKind};
use drake_types::token::{Symbol, Token};
use somen::prelude::*;

use crate::expression::expression;
use crate::pattern::pattern;
use crate::token::{spaces, symbol};

/// A parser for statements
pub fn statement<'a, I>() -> impl Parser<I, Output = Statement<I::Locator>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    choice((
        value_binding().map(|(pat, expr)| StatementKind::ValueBinding(pat, expr)),
        table_header().map(|(kind, pat, expr)| StatementKind::TableHeader(kind, pat, expr)),
    ))
    .with_position()
    .map(|(kind, span)| Statement { kind, span })
}

/// A parser for value bindings
pub fn value_binding<'a, I>(
) -> impl Parser<I, Output = (Pattern<I::Locator>, Expression<I::Locator>)> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    pattern()
        .skip((spaces(false), symbol(Symbol::Assign), spaces(false)))
        .and(expression())
}

/// A parser for table headers
pub fn table_header<'a, I>() -> impl Parser<
    I,
    Output = (
        TableHeaderKind,
        Pattern<I::Locator>,
        Option<Expression<I::Locator>>,
    ),
> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    symbol(Symbol::OpenBracket)
        .skip(spaces(true))
        .prefix(symbol(Symbol::OpenBracket).skip(spaces(true)).opt())
        .then(|sym| {
            table_header_inner()
                .skip(
                    (spaces(true), symbol(Symbol::CloseBracket))
                        .times(if sym.is_some() { 2 } else { 1 })
                        .discard(),
                )
                .map(move |(pat, expr)| {
                    (
                        if sym.is_some() {
                            TableHeaderKind::Array
                        } else {
                            TableHeaderKind::Normal
                        },
                        pat,
                        expr,
                    )
                })
        })
}

pub fn table_header_inner<'a, I>(
) -> impl Parser<I, Output = (Pattern<I::Locator>, Option<Expression<I::Locator>>)> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    (
        pattern(),
        (spaces(true), symbol(Symbol::Assign), spaces(true))
            .prefix(expression())
            .opt(),
    )
}
