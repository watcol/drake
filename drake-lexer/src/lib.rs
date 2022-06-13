#[cfg(test)]
mod tests;

pub mod key;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use somen::prelude::*;

use key::{key, Key};
use literal::{literal, Literal};
use space::{comment, continuous, newline, whitespaces};
use symbol::{symbol, Symbol};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Newline,
    Comment(String),
    Symbol(Symbol),
    Key(Key),
    Literal(Literal),
}

pub fn token<'a, I>() -> impl Parser<I, Output = Token> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        newline().map(|_| Token::Newline),
        comment().map(Token::Comment),
        symbol().map(Token::Symbol),
        key().map(Token::Key),
        literal().map(Token::Literal),
    ))
    .expect("token")
}

pub fn tokens<'a, I>() -> impl IterableParser<I, Item = Token> + 'a
where
    I: Input<Ok = char> + 'a,
{
    whitespaces().prefix(
        choice_iterable((
            token().skip(whitespaces()).once(),
            continuous()
                .map(Token::Comment)
                .chain(token().skip(whitespaces()).once()),
        ))
        .flat_repeat(..),
    )
}
