//! Parsers for patterns
#[cfg(test)]
mod tests;

use drake_types::ast::{Pattern, PatternKind};
use drake_types::token::Token;
use somen::prelude::*;

use crate::key::key;

/// A parser for patterns
pub fn pattern<'a, I>() -> impl Parser<I, Output = Pattern<I::Locator>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    choice((key().map(PatternKind::Key),))
        .with_position()
        .map(|(kind, span)| Pattern { kind, span })
}
