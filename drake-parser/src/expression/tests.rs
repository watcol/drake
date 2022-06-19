use alloc::string::String;
use alloc::vec;
use drake_types::ast::{Expression, ExpressionKind, Key, KeyKind, Literal};
use drake_types::token::{Identifier, IdentifierKind, Literal as TokenLit, Symbol, Token};
use somen::prelude::*;

use crate::test_utils::test_parser;

#[test]
fn expression() {
    test_parser(
        super::expression().complete(),
        &[
            (
                &[Token::Literal(TokenLit::Character('a'))],
                Some(Expression {
                    kind: ExpressionKind::Literal(Literal::Character('a')),
                    span: 0..1,
                }),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(Expression {
                    kind: ExpressionKind::Array(vec![]),
                    span: 0..2,
                }),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                Some(Expression {
                    kind: ExpressionKind::InlineTable(vec![]),
                    span: 0..2,
                }),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn array() {
    test_parser(
        super::array().complete(),
        &[
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(vec![]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(vec![]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(vec![Expression {
                    kind: ExpressionKind::Literal(Literal::Character('a')),
                    span: 1..2,
                }]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Symbol(Symbol::Comma),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(vec![Expression {
                    kind: ExpressionKind::Literal(Literal::Character('a')),
                    span: 1..2,
                }]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Symbol(Symbol::Comma),
                    Token::Literal(TokenLit::Character('b')),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(vec![
                    Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 1..2,
                    },
                    Expression {
                        kind: ExpressionKind::Literal(Literal::Character('b')),
                        span: 3..4,
                    },
                ]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Literal(TokenLit::Character('b')),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                None,
            ),
            (
                &[
                    Token::Whitespaces,
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Symbol(Symbol::CloseBracket),
                    Token::Newline,
                ],
                None,
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn inline_table() {
    test_parser(
        super::inline_table().complete(),
        &[
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                Some(vec![]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBrace),
                ],
                Some(vec![]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::Assign),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Literal(TokenLit::Character('a')),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::Comma),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBrace),
                ],
                Some(vec![(
                    Key {
                        kind: KeyKind::Normal,
                        name: String::from("abc"),
                        span: 1..2,
                    },
                    Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 7..8,
                    },
                )]),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::Comma),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Literal(TokenLit::Character('a')),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("def"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Literal(TokenLit::Character('b')),
                    Token::Symbol(Symbol::CloseBrace),
                ],
                None,
            ),
            (
                &[
                    Token::Whitespaces,
                    Token::Symbol(Symbol::OpenBrace),
                    Token::Whitespaces,
                    Token::Symbol(Symbol::CloseBrace),
                    Token::Newline,
                ],
                None,
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}
