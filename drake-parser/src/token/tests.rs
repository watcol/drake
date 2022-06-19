use crate::test_utils::test_parser;
use alloc::string::String;
use drake_types::{
    ast::Literal,
    token::{Identifier, IdentifierKind, Literal as TokenLit, Radix, StringKind, Symbol, Token},
};
use somen::prelude::*;

#[test]
fn spaces() {
    test_parser(
        super::spaces().complete(),
        &[
            (&[Token::Whitespaces], Some(())),
            (&[Token::Comment(String::from("abc"))], Some(())),
            (
                &[
                    Token::Whitespaces,
                    Token::Comment(String::from("abc")),
                    Token::Whitespaces,
                ],
                Some(()),
            ),
            (
                &[Token::Symbol(Symbol::BackSlash), Token::Newline],
                Some(()),
            ),
            (
                &[
                    Token::Whitespaces,
                    Token::Symbol(Symbol::BackSlash),
                    Token::Newline,
                    Token::Comment(String::from("abc")),
                    Token::Whitespaces,
                    Token::Symbol(Symbol::BackSlash),
                    Token::Newline,
                ],
                Some(()),
            ),
            (&[Token::Newline], None),
            (&[Token::Whitespaces, Token::Symbol(Symbol::Assign)], None),
            (&[Token::Symbol(Symbol::BackSlash)], None),
            (
                &[
                    Token::Symbol(Symbol::BackSlash),
                    Token::Newline,
                    Token::Symbol(Symbol::Assign),
                ],
                None,
            ),
        ],
    );
}

#[test]
fn newline() {
    test_parser(
        super::newline().complete(),
        &[
            (&[Token::Newline], Some(())),
            (&[Token::Whitespaces, Token::Newline], Some(())),
            (
                &[
                    Token::Whitespaces,
                    Token::Comment(String::from("abc")),
                    Token::Whitespaces,
                    Token::Newline,
                ],
                Some(()),
            ),
        ],
    );
}

#[test]
fn literal() {
    test_parser(
        super::literal().complete(),
        &[
            (
                &[Token::Literal(TokenLit::Character('a'))],
                Some(Literal::Character('a')),
            ),
            (
                &[Token::Literal(TokenLit::String(
                    String::from("abc"),
                    StringKind::Normal,
                ))],
                Some(Literal::String(String::from("abc"))),
            ),
            (
                &[Token::Literal(TokenLit::Integer(0, Radix::Decimal))],
                Some(Literal::Integer(0)),
            ),
            (
                &[Token::Literal(TokenLit::Float(0.0))],
                Some(Literal::Float(0.0)),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn symbol() {
    test_parser(
        super::symbol(Symbol::Assign).complete(),
        &[
            (&[Token::Symbol(Symbol::Assign)], Some(())),
            (&[Token::Symbol(Symbol::Comma)], None),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn identifier() {
    test_parser(
        super::identifier().complete(),
        &[
            (
                &[Token::Identifier(Identifier {
                    kind: IdentifierKind::Bare,
                    name: String::from("abc"),
                })],
                Some(String::from("abc")),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}
