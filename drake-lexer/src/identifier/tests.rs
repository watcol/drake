use alloc::string::String;
use drake_types::token::{Identifier, IdentifierKind};
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn identifier() {
    block_on(async {
        let parser = &mut super::identifier().complete();
        assert_parser(
            parser,
            "abc",
            Identifier {
                kind: IdentifierKind::Bare,
                name: String::from("abc"),
            },
        )
        .await;
        assert_parser(
            parser,
            "${abc}",
            Identifier {
                kind: IdentifierKind::Raw,
                name: String::from("abc"),
            },
        )
        .await;
    })
}

#[test]
fn bare_key() {
    block_on(async {
        let parser = &mut super::bare_key().complete();
        assert_parser(parser, "a", String::from("a")).await;
        assert_parser(parser, "A", String::from("A")).await;
        assert_parser(parser, "abc_012", String::from("abc_012")).await;
        assert_parser_fail(parser, "_abc").await;
        assert_parser_fail(parser, "0ad").await;
    })
}

#[test]
fn raw_key() {
    block_on(async {
        let parser = &mut super::raw_key().complete();
        assert_parser(parser, "${}", String::from("")).await;
        assert_parser(parser, "${A b \r\n}", String::from("A b \n")).await;
        assert_parser(
            parser,
            "${\\\\{All\\u{00A0}characters\\ncan be used.\\}}",
            String::from("\\{All\u{00A0}characters\ncan be used.}"),
        )
        .await;
    })
}
