#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod identifier;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use drake_types::token::Token;
use somen::prelude::*;

use identifier::identifier;
use literal::literal;
use space::{comment, newline, whitespaces};
use symbol::symbol;

/// A parser for tokens
pub fn token<'a, I>() -> impl Parser<I, Output = Token> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        newline().map(|_| Token::Newline),
        whitespaces().map(|_| Token::Whitespaces),
        comment().map(Token::Comment),
        symbol().map(Token::Symbol),
        identifier().map(Token::Identifier),
        literal().map(Token::Literal),
    ))
    .expect("token")
}
