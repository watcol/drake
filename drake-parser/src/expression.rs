//! Parsers for expressions
#[cfg(test)]
mod tests;

use drake_types::ast::{Expression, ExpressionKind};
use drake_types::token::Token;
use somen::prelude::*;

use crate::token::literal;

/// A parser for patterns
pub fn expression<'a, I>() -> impl Parser<I, Output = Expression<I::Locator>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    choice((literal().map(ExpressionKind::Literal),))
        .with_position()
        .map(|(kind, span)| Expression { kind, span })
}
