//! Parsers for keys
#[cfg(test)]
mod tests;

use drake_types::ast::{Key, KeyKind};
use drake_types::token::{Symbol, Token};
use somen::prelude::*;

use crate::token::{identifier, symbol};

/// A parser for keys
pub fn key<'a, I>() -> impl Parser<I, Output = Key<I::Locator>> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    let prefix = choice((
        symbol(Symbol::Underscore).map(|_| KeyKind::Local),
        symbol(Symbol::At).map(|_| KeyKind::Builtin),
        value(KeyKind::Normal),
    ));

    (prefix, identifier())
        .with_position()
        .map(|((kind, name), span)| Key { kind, name, span })
}
