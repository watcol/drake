use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn whitespaces() {
    block_on(async {
        let parser = &mut super::whitespaces().complete();
        assert_parser(parser, "", ()).await;
        assert_parser(parser, " ", ()).await;
        assert_parser(parser, "\t", ()).await;
        assert_parser(parser, " \t ", ()).await;
        assert_parser(parser, "  \t\t", ()).await;
        assert_parser_fail(parser, "\t\n\t").await;
    })
}

#[test]
fn continuous() {
    block_on(async {
        let parser = &mut super::continuous().collect().complete();
        assert_parser(parser, "\\\n", vec![]).await;
        assert_parser(parser, "\\ # Comment\n", vec![String::from(" Comment")]).await;
        assert_parser(
            parser,
            "\\ # Comment\n# Comment\n",
            vec![String::from(" Comment"), String::from(" Comment")],
        )
        .await;
        assert_parser(parser, "\\\n # Comment\n", vec![String::from(" Comment")]).await;
        assert_parser(parser, "\\ # Comment", vec![String::from(" Comment")]).await;
        assert_parser(parser, "\\\n # Comment", vec![String::from(" Comment")]).await;
        assert_parser_fail(parser, "\\").await;
    })
}

#[test]
fn newline() {
    block_on(async {
        let parser = &mut super::newline().complete();
        assert_parser(parser, "\n \t\r\n\n", ()).await;
        assert_parser_fail(parser, "\n \t\r#\n\n").await;
    })
}

#[test]
fn comment() {
    block_on(async {
        let parser = &mut super::comment().complete();
        assert_parser(parser, "## Comment", String::from("# Comment")).await;
        assert_parser_fail(parser, "##\r Comment").await;
        assert_parser_fail(parser, "## Comment\n").await;
    })
}
