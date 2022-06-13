//! Symbol tokens
#[cfg(test)]
mod tests;

use drake_types::token::Symbol;
use somen::prelude::*;

/// A parser for symbols
pub fn symbol<'a, I>() -> impl Parser<I, Output = Symbol> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        token('=').map(|_| Symbol::Assign),
        token(',').map(|_| Symbol::Comma),
        token('.').map(|_| Symbol::Dot),
        token('\\').map(|_| Symbol::BackSlash),
        token('[').map(|_| Symbol::OpenBracket),
        token(']').map(|_| Symbol::CloseBracket),
        token('{').map(|_| Symbol::OpenBrace),
        token('}').map(|_| Symbol::CloseBrace),
    ))
    .expect("symbol")
}
