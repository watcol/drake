#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod key;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use somen::prelude::*;
use drake_types::token::{Token, TokenValue};

use key::key;
use literal::literal;
use space::{comment, continuous, newline, whitespaces};
use symbol::symbol;


/// A parser for tokens
pub fn token<'a, I>() -> impl Parser<I, Output = Token> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    choice((
        newline().map(|_| TokenValue::Newline),
        comment().map(TokenValue::Comment),
        symbol().map(TokenValue::Symbol),
        key().map(TokenValue::Key),
        literal().map(TokenValue::Literal),
    ))
    .with_position()
    .map(|(kind, pos)| Token { kind, pos })
    .expect("token")
}

/// An iterable parser for sequences of tokens.
pub fn tokens<'a, I>() -> impl IterableParser<I, Item = Token> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    whitespaces().prefix(
        choice_iterable((
            token().skip(whitespaces()).once(),
            continuous()
                .map(|(c, pos)| Token {
                    kind: TokenValue::Comment(c),
                    pos,
                })
                .chain(token().skip(whitespaces()).once()),
        ))
        .flat_repeat(..),
    )
}
