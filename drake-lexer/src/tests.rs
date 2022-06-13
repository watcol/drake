use futures_executor::block_on;
use somen::prelude::*;

use super::{Token, TokenKind};
use crate::key::Key;
use crate::literal::Literal;
use crate::symbol::Symbol;
use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn token() {
    block_on(async {
        let parser = &mut super::token().map(|Token { kind, .. }| kind).complete();
        assert_parser(parser, "\n", TokenKind::Newline).await;
        assert_parser(parser, "#abc", TokenKind::Comment(String::from("abc"))).await;
        assert_parser(parser, "=", TokenKind::Symbol(Symbol::Assign)).await;
        assert_parser(
            parser,
            "abc",
            TokenKind::Key(Key::Normal(String::from("abc"))),
        )
        .await;
        assert_parser(parser, "0", TokenKind::Literal(Literal::Integer(0))).await;
    })
}

#[test]
fn tokens() {
    block_on(async {
        let parser = &mut super::tokens()
            .map(|Token { kind, .. }| kind)
            .collect()
            .complete();
        assert_parser(
            parser,
            " = \n # abc \n 0 # def ",
            vec![
                TokenKind::Symbol(Symbol::Assign),
                TokenKind::Newline,
                TokenKind::Comment(String::from(" abc ")),
                TokenKind::Newline,
                TokenKind::Literal(Literal::Integer(0)),
                TokenKind::Comment(String::from(" def ")),
            ],
        )
        .await;
        assert_parser(
            parser,
            " = \\ # abc \n # def \n 0",
            vec![
                TokenKind::Symbol(Symbol::Assign),
                TokenKind::Comment(String::from(" abc ")),
                TokenKind::Comment(String::from(" def ")),
                TokenKind::Literal(Literal::Integer(0)),
            ],
        )
        .await;
        assert_parser_fail(parser, " = \\\n").await;
    })
}
