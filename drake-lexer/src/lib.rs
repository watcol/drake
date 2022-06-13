#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests;

pub mod key;
pub mod literal;
pub mod space;
pub mod symbol;
mod utils;

use alloc::string::String;
use core::ops::Range;
use somen::prelude::*;

use key::{key, Key};
use literal::{literal, Literal};
use space::{comment, continuous, newline, whitespaces};
use symbol::{symbol, Symbol};

/// Kinds of tokens
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Newline,
    Comment(String),
    Symbol(Symbol),
    Key(Key),
    Literal(Literal),
}

/// A token value and a position
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

/// A parser for tokens
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
                    kind: TokenKind::Comment(c),
                    pos,
                })
                .chain(token().skip(whitespaces()).once()),
        ))
        .flat_repeat(..),
    )
}
