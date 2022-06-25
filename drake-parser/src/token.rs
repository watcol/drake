//! Parsers for basic tokens
#[cfg(test)]
mod tests;

use alloc::string::String;
use drake_types::ast::Literal;
use drake_types::token::{Literal as TokenLit, Symbol, Token};
use somen::prelude::*;

/// Spaces and line continuouses
pub fn spaces<'a, I>(in_bracket: bool) -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    choice((
        is(move |token| {
            if in_bracket {
                matches!(
                    token,
                    Token::Newline | Token::Whitespaces | Token::Comment(_)
                )
            } else {
                matches!(token, Token::Whitespaces | Token::Comment(_))
            }
        })
        .discard(),
        (symbol(Symbol::BackSlash), newline()).discard(),
    ))
    .expect("space")
    .repeat(..)
    .discard()
}

/// A newline
pub fn newline<'a, I>() -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = Token> + 'a,
{
    is(|token| matches!(token, Token::Whitespaces | Token::Comment(_)))
        .repeat(..)
        .discard()
        .skip(is(|token| *token == Token::Newline))
        .expect("newline")
}

/// A literal
pub fn literal<'a, I>() -> impl Parser<I, Output = Literal> + 'a
where
    I: Positioned<Ok = Token> + 'a,
{
    is_some(|token| match token {
        Token::Literal(TokenLit::Character(c)) => Some(Literal::Character(c)),
        Token::Literal(TokenLit::String(s, _)) => Some(Literal::String(s)),
        Token::Literal(TokenLit::Integer(i, _)) => Some(Literal::Integer(i)),
        Token::Literal(TokenLit::Float(f)) => Some(Literal::Float(f)),
        _ => None,
    })
    .expect("literal")
}

/// A specified symbol
pub fn symbol<'a, I>(symbol: Symbol) -> impl Parser<I, Output = ()> + 'a
where
    I: Positioned<Ok = Token> + 'a,
{
    token(Token::Symbol(symbol)).discard()
}

/// An identifier
pub fn identifier<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Positioned<Ok = Token> + 'a,
{
    is_some(|token| {
        if let Token::Identifier(ident) = token {
            Some(ident.name)
        } else {
            None
        }
    })
    .expect("identifier")
}
