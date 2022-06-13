use alloc::string::String;
use futures_executor::block_on;
use somen::prelude::*;

use super::Literal;
use crate::utils::assert_parser;

#[test]
fn literal() {
    block_on(async {
        let parser = &mut super::literal().complete();
        assert_parser(parser, "0.0", Literal::Float(0.0)).await;
        assert_parser(parser, "1e3", Literal::Float(1e3)).await;
        assert_parser(parser, "0", Literal::Integer(0)).await;
        assert_parser(parser, "'0'", Literal::Character('0')).await;
        assert_parser(parser, "\"0\"", Literal::String(String::from("0"))).await;
        assert_parser(parser, "\"\"\"0\"\"\"", Literal::String(String::from("0"))).await;
    })
}
