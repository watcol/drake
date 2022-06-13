//! Symbol tokens
#[cfg(test)]
mod tests;

use somen::prelude::*;

/// Kinds of symbols
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Assign,
    Comma,
    Dot,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

/// A parser for symbols
pub fn symbol<'a, I>() -> impl Parser<I, Output = Symbol> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        token('=').map(|_| Symbol::Assign),
        token(',').map(|_| Symbol::Comma),
        token('.').map(|_| Symbol::Dot),
        token('[').map(|_| Symbol::OpenBracket),
        token(']').map(|_| Symbol::CloseBracket),
        token('{').map(|_| Symbol::OpenBrace),
        token('}').map(|_| Symbol::CloseBrace),
    ))
    .expect("symbol")
}
