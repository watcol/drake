#[cfg(test)]
mod tests;

pub mod key;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use core::ops::Range;
use somen::prelude::*;

use key::{key, Key};
use literal::{literal, Literal};
use space::{comment, continuous, newline, whitespaces};
use symbol::{symbol, Symbol};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Newline,
    Comment(String),
    Symbol(Symbol),
    Key(Key),
    Literal(Literal),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Range<usize>,
}

impl PartialEq<Token> for TokenKind {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        self.eq(&other.kind)
    }
}

impl PartialEq<TokenKind> for Token {
    #[inline]
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind.eq(other)
    }
}

impl PartialEq for Token {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

pub fn token<'a, I>() -> impl Parser<I, Output = Token> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    choice((
        newline().map(|_| TokenKind::Newline),
        comment().map(TokenKind::Comment),
        symbol().map(TokenKind::Symbol),
        key().map(TokenKind::Key),
        literal().map(TokenKind::Literal),
    ))
    .with_position()
    .map(|(kind, pos)| Token { kind, pos })
    .expect("token")
}

pub fn tokens<'a, I>() -> impl IterableParser<I, Item = Token> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    whitespaces().prefix(
        choice_iterable((
            token().skip(whitespaces()).once(),
            continuous()
                .map(|(c, pos)| Token {
                    kind: TokenKind::Comment(c),
                    pos,
                })
                .chain(token().skip(whitespaces()).once()),
        ))
        .flat_repeat(..),
    )
}
