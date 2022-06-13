use alloc::string::String;
use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn whitespaces() {
    block_on(async {
        let parser = &mut super::whitespaces().complete();
        assert_parser(parser, " ", ()).await;
        assert_parser(parser, "\t", ()).await;
        assert_parser(parser, " \t ", ()).await;
        assert_parser(parser, "  \t\t", ()).await;
        assert_parser_fail(parser, "").await;
        assert_parser_fail(parser, "\t\n\t").await;
    })
}

#[test]
fn newline() {
    block_on(async {
        let parser = &mut super::newline().complete();
        assert_parser(parser, "\n", '\n').await;
        assert_parser(parser, "\r", '\n').await;
        assert_parser(parser, "\r\n", '\n').await;
        assert_parser_fail(parser, "\n\r").await;
    })
}

#[test]
fn comment() {
    block_on(async {
        let parser = &mut super::comment().complete();
        assert_parser(parser, "## Comment", String::from("# Comment")).await;
        assert_parser_fail(parser, "#\r Comment").await;
        assert_parser_fail(parser, "# Comment\n").await;
    })
}
