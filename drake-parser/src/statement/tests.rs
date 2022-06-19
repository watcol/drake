use alloc::string::String;
use drake_types::ast::{
    Expression, ExpressionKind, Key, KeyKind, Literal, Pattern, PatternKind, Statement,
    StatementKind, TableHeaderKind,
};
use drake_types::token::{Identifier, IdentifierKind, Literal as LitToken, Symbol, Token};
use somen::prelude::*;

use crate::test_utils::test_parser;

#[test]
fn statement() {
    test_parser(
        super::statement().complete(),
        &[
            (
                &[
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Literal(LitToken::Character('a')),
                ],
                Some(Statement {
                    kind: StatementKind::ValueBinding(
                        Pattern {
                            kind: PatternKind::Key(Key {
                                kind: KeyKind::Normal,
                                name: String::from("abc"),
                                span: 0..1,
                            }),
                            span: 0..1,
                        },
                        Expression {
                            kind: ExpressionKind::Literal(Literal::Character('a')),
                            span: 2..3,
                        },
                    ),
                    span: 0..3,
                }),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some(Statement {
                    kind: StatementKind::TableHeader(
                        TableHeaderKind::Normal,
                        Pattern {
                            kind: PatternKind::Key(Key {
                                kind: KeyKind::Normal,
                                name: String::from("abc"),
                                span: 1..2,
                            }),
                            span: 1..2,
                        },
                        None,
                    ),
                    span: 0..3,
                }),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn value_binding() {
    test_parser(
        super::value_binding().complete(),
        &[
            (
                &[
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Literal(LitToken::Character('a')),
                ],
                Some((
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 0..1,
                        }),
                        span: 0..1,
                    },
                    Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 2..3,
                    },
                )),
            ),
            (
                &[
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Whitespaces,
                    Token::Symbol(Symbol::Assign),
                    Token::Whitespaces,
                    Token::Literal(LitToken::Character('a')),
                ],
                Some((
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 0..1,
                        }),
                        span: 0..1,
                    },
                    Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 4..5,
                    },
                )),
            ),
            (
                &[
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::Assign),
                    Token::Newline,
                    Token::Literal(LitToken::Character('a')),
                ],
                None,
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}

#[test]
fn table_header() {
    test_parser(
        super::table_header().complete(),
        &[
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some((
                    TableHeaderKind::Normal,
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 1..2,
                        }),
                        span: 1..2,
                    },
                    None,
                )),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::Assign),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Literal(LitToken::Character('a')),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some((
                    TableHeaderKind::Normal,
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 3..4,
                        }),
                        span: 3..4,
                    },
                    Some(Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 9..10,
                    }),
                )),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Symbol(Symbol::CloseBracket),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some((
                    TableHeaderKind::Array,
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 2..3,
                        }),
                        span: 2..3,
                    },
                    None,
                )),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::Assign),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Literal(LitToken::Character('a')),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBracket),
                    Token::Whitespaces,
                    Token::Newline,
                    Token::Symbol(Symbol::CloseBracket),
                ],
                Some((
                    TableHeaderKind::Array,
                    Pattern {
                        kind: PatternKind::Key(Key {
                            kind: KeyKind::Normal,
                            name: String::from("abc"),
                            span: 6..7,
                        }),
                        span: 6..7,
                    },
                    Some(Expression {
                        kind: ExpressionKind::Literal(Literal::Character('a')),
                        span: 12..13,
                    }),
                )),
            ),
            (
                &[
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Symbol(Symbol::CloseBracket),
                ],
                None,
            ),
            (
                &[
                    Token::Whitespaces,
                    Token::Symbol(Symbol::OpenBracket),
                    Token::Whitespaces,
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                    Token::Whitespaces,
                    Token::Symbol(Symbol::CloseBracket),
                    Token::Whitespaces,
                ],
                None,
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}
