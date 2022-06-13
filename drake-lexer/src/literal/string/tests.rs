use futures_executor::block_on;
use alloc::string::String;
use somen::prelude::*;

use crate::utils::assert_parser;

#[test]
fn character() {
    block_on(async {
        let parser = &mut super::character().complete();
        assert_parser(parser, "'a'", 'a').await;
    })
}

#[test]
fn string() {
    block_on(async {
        let parser = &mut super::string().complete();
        assert_parser(parser, "\"abc\"", String::from("abc")).await;
        assert_parser(parser, "\"\\\n\r\n\r\"", String::from("\n\n")).await;
    })
}

#[test]
fn raw_string() {
    block_on(async {
        let parser = &mut super::raw_string().complete();
        assert_parser(
            parser,
            "\"\"\"\\ServerX\\admin$\\system32\\\"\"\"",
            String::from("\\ServerX\\admin$\\system32\\"),
        )
        .await;
        assert_parser(parser, "\"\"\"\"(\"\"\")\"\"\"\"", String::from("(\"\"\")")).await;
    })
}
