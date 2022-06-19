//! Key(identifier) tokens
#[cfg(test)]
mod tests;

use alloc::string::String;
use drake_types::token::{Identifier, IdentifierKind};
use somen::prelude::*;

use crate::utils::escaped_char;

/// A parser for normal (bare and raw) keys
pub fn identifier<'a, I>() -> impl Parser<I, Output = Identifier> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        bare_key().map(|name| Identifier {
            kind: IdentifierKind::Bare,
            name,
        }),
        raw_key().map(|name| Identifier {
            kind: IdentifierKind::Raw,
            name,
        }),
    ))
    .expect("identifier")
}

/// A parser for bare keys
pub fn bare_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    (
        is(char::is_ascii_alphabetic).expect("[A-Za-z]").once(),
        is(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .expect("[0-9_A-Za-z]")
            .repeat(..),
    )
        .collect()
        .expect("bare key")
}

/// A parser for raw keys
pub fn raw_key<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char('}')
        .expect("character")
        .repeat(..)
        .collect()
        .between(tag("${"), token('}'))
        .expect("raw key")
}
