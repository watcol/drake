use futures_executor::block_on;
use somen::prelude::*;

use super::Token;
use crate::key::Key;
use crate::literal::Literal;
use crate::symbol::Symbol;
use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn token() {
    block_on(async {
        let parser = &mut super::token().complete();
        assert_parser(parser, "\n", Token::Newline).await;
        assert_parser(parser, "#abc", Token::Comment(String::from("abc"))).await;
        assert_parser(parser, "=", Token::Symbol(Symbol::Assign)).await;
        assert_parser(parser, "abc", Token::Key(Key::Normal(String::from("abc")))).await;
        assert_parser(parser, "0", Token::Literal(Literal::Integer(0))).await;
    })
}

#[test]
fn tokens() {
    block_on(async {
        let parser = &mut super::tokens().collect().complete();
        assert_parser(
            parser,
            " = \n # abc \n 0 # def ",
            vec![
                Token::Symbol(Symbol::Assign),
                Token::Newline,
                Token::Comment(String::from(" abc ")),
                Token::Newline,
                Token::Literal(Literal::Integer(0)),
                Token::Comment(String::from(" def ")),
            ],
        )
        .await;
        assert_parser(
            parser,
            " = \\ # abc \n # def \n 0",
            vec![
                Token::Symbol(Symbol::Assign),
                Token::Comment(String::from(" abc ")),
                Token::Comment(String::from(" def ")),
                Token::Literal(Literal::Integer(0)),
            ],
        )
        .await;
        assert_parser_fail(parser, " = \\\n").await;
    })
}
