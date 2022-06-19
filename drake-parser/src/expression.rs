//! Parsers for expressions
#[cfg(test)]
mod tests;

use alloc::vec::Vec;
use drake_types::ast::{Expression, ExpressionKind, Key};
use drake_types::token::{Symbol, Token};
use somen::{call, prelude::*};

use crate::key::key;
use crate::token::{literal, spaces, symbol};

/// A parser for expression
pub fn expression<'a, I>() -> impl Parser<I, Output = Expression<I::Locator>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    choice((
        literal().map(ExpressionKind::Literal),
        array().map(ExpressionKind::Array),
        inline_table().map(ExpressionKind::InlineTable),
    ))
    .with_position()
    .map(|(kind, span)| Expression { kind, span })
}

/// A parser for arrays
pub fn array<'a, I>() -> impl Parser<I, Output = Vec<Expression<I::Locator>>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    call!(expression)
        .skip(spaces(true))
        .sep_by_end(symbol(Symbol::Comma).skip(spaces(true)), ..)
        .between(
            symbol(Symbol::OpenBracket).skip(spaces(true)),
            symbol(Symbol::CloseBracket),
        )
        .collect()
}

/// A parser for inline tables
pub fn inline_table<'a, I>(
) -> impl Parser<I, Output = Vec<(Key<I::Locator>, Expression<I::Locator>)>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    key()
        .skip((spaces(true), symbol(Symbol::Assign), spaces(true)))
        .and(call!(expression))
        .skip(spaces(true))
        .sep_by_end(symbol(Symbol::Comma).skip(spaces(true)), ..)
        .between(
            symbol(Symbol::OpenBrace).skip(spaces(true)),
            symbol(Symbol::CloseBrace),
        )
        .collect()
}
