use futures_executor::block_on;
use somen::prelude::*;

use crate::utils::{assert_parser, assert_parser_fail};

#[test]
fn digits() {
    block_on(async {
        let parser = &mut super::digits(16).collect().complete();
        assert_parser(parser, "0", String::from("0")).await;
        assert_parser(parser, "10", String::from("10")).await;
        assert_parser(parser, "42", String::from("42")).await;
        assert_parser(parser, "aF", String::from("aF")).await;
        assert_parser(parser, "4_2", String::from("42")).await;
        assert_parser(parser, "4__2", String::from("42")).await;
        assert_parser(parser, "42_", String::from("42")).await;
        assert_parser_fail(parser, "042").await;
        assert_parser_fail(parser, "_42").await;
    });
}

#[test]
fn digits_trailing_zeros() {
    block_on(async {
        let parser = &mut super::digits_trailing_zeros(16).collect().complete();
        assert_parser(parser, "0", String::from("0")).await;
        assert_parser(parser, "10", String::from("10")).await;
        assert_parser(parser, "42", String::from("42")).await;
        assert_parser(parser, "042", String::from("042")).await;
        assert_parser(parser, "aF", String::from("aF")).await;
        assert_parser(parser, "4_2", String::from("42")).await;
        assert_parser(parser, "4__2", String::from("42")).await;
        assert_parser(parser, "42_", String::from("42")).await;
        assert_parser_fail(parser, "_42").await;
    });
}

