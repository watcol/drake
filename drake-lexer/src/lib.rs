#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod key;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use drake_types::token::TokenValue;
use somen::prelude::*;

use key::key;
use literal::literal;
use space::{comment, newline, whitespaces};
use symbol::symbol;

/// A parser for tokens
pub fn token<'a, I>() -> impl Parser<I, Output = TokenValue> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    choice((
        newline().map(|_| TokenValue::Newline),
        whitespaces().map(|_| TokenValue::Whitespaces),
        comment().map(TokenValue::Comment),
        symbol().map(TokenValue::Symbol),
        key().map(TokenValue::Key),
        literal().map(TokenValue::Literal),
    ))
    .expect("token")
}
