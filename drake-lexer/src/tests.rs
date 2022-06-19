use alloc::string::String;
use drake_types::token::{Identifier, IdentifierKind, Literal, Radix, Symbol, Token};
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::assert_parser;

#[test]
fn token() {
    block_on(async {
        let parser = &mut super::token().complete();
        assert_parser(parser, "\n", Token::Newline).await;
        assert_parser(parser, " ", Token::Whitespaces).await;
        assert_parser(parser, "#abc", Token::Comment(String::from("abc"))).await;
        assert_parser(parser, "=", Token::Symbol(Symbol::Assign)).await;
        assert_parser(
            parser,
            "abc",
            Token::Identifier(Identifier {
                kind: IdentifierKind::Bare,
                name: String::from("abc"),
            }),
        )
        .await;
        assert_parser(
            parser,
            "0",
            Token::Literal(Literal::Integer(0, Radix::Decimal)),
        )
        .await;
    })
}
