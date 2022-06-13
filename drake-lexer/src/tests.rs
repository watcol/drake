use alloc::string::String;
use drake_types::token::{Key, Literal, Symbol, TokenValue};
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::assert_parser;

#[test]
fn token() {
    block_on(async {
        let parser = &mut super::token().complete();
        assert_parser(parser, "\n", TokenValue::Newline).await;
        assert_parser(parser, " ", TokenValue::Whitespaces).await;
        // assert_parser(parser, "#abc", TokenValue::Comment(String::from("abc"))).await;
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
