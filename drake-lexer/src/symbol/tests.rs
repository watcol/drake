use drake_types::token::Symbol;
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn symbol() {
    block_on(async {
        let parser = &mut super::symbol().complete();
        assert_parser(parser, "=", Symbol::Assign).await;
        assert_parser(parser, ",", Symbol::Comma).await;
        assert_parser(parser, ".", Symbol::Dot).await;
        assert_parser(parser, "\\", Symbol::BackSlash).await;
        assert_parser(parser, "_", Symbol::Underscore).await;
        assert_parser(parser, "@", Symbol::At).await;
        assert_parser(parser, "[", Symbol::OpenBracket).await;
        assert_parser(parser, "]", Symbol::CloseBracket).await;
        assert_parser(parser, "{", Symbol::OpenBrace).await;
        assert_parser(parser, "}", Symbol::CloseBrace).await;
        assert_parser_fail(parser, "(").await;
    })
}
