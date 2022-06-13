use alloc::string::String;
use alloc::vec;
use drake_types::token::{Key, Literal, Symbol, Token, TokenValue};
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn token() {
    block_on(async {
        let parser = &mut super::token().map(|Token { kind, .. }| kind).complete();
        assert_parser(parser, "\n", TokenValue::Newline).await;
        assert_parser(parser, "#abc", TokenValue::Comment(String::from("abc"))).await;
        assert_parser(parser, "=", TokenValue::Symbol(Symbol::Assign)).await;
        assert_parser(
            parser,
            "abc",
            TokenValue::Key(Key::Normal(String::from("abc"))),
        )
        .await;
        assert_parser(parser, "0", TokenValue::Literal(Literal::Integer(0))).await;
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
                TokenValue::Symbol(Symbol::Assign),
                TokenValue::Newline,
                TokenValue::Comment(String::from(" abc ")),
                TokenValue::Newline,
                TokenValue::Literal(Literal::Integer(0)),
                TokenValue::Comment(String::from(" def ")),
            ],
        )
        .await;
        assert_parser(
            parser,
            " = \\ # abc \n # def \n 0",
            vec![
                TokenValue::Symbol(Symbol::Assign),
                TokenValue::Comment(String::from(" abc ")),
                TokenValue::Comment(String::from(" def ")),
                TokenValue::Literal(Literal::Integer(0)),
            ],
        )
        .await;
        assert_parser_fail(parser, " = \\\n").await;
    })
}
