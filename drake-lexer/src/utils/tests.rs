use super::{assert_parser, assert_parser_fail};
use futures_executor::block_on;
use somen::prelude::*;

#[test]
fn escaped_char() {
    block_on(async {
        let parser = &mut super::escaped_char('\'').complete();
        assert_parser(parser, "a", 'a').await;
        assert_parser(parser, "\n", '\n').await;
        assert_parser(parser, "\\n", '\n').await;
        assert_parser(parser, "\\r", '\r').await;
        assert_parser(parser, "\\t", '\t').await;
        assert_parser(parser, "\\\\", '\\').await;
        assert_parser(parser, "\\'", '\'').await;
        assert_parser(parser, "\\x00", '\x00').await;
        assert_parser(parser, "\\x0A", '\x0A').await;
        assert_parser(parser, "\\x7f", '\x7f').await;
        assert_parser(parser, "\\u{Ab}", '\u{AB}').await;
        assert_parser(parser, "\\u{00a0}", '\u{A0}').await;
        assert_parser(parser, "\\u{D7FF}", '\u{D7FF}').await;
        assert_parser(parser, "\\u{E000}", '\u{E000}').await;
        assert_parser(parser, "\\u{10FFFF}", '\u{10FFFF}').await;
        assert_parser_fail(parser, "'").await;
        assert_parser_fail(parser, "\\").await;
        assert_parser_fail(parser, "\\x80").await;
        assert_parser_fail(parser, "\\U{00}").await;
        assert_parser_fail(parser, "\\u{D800}").await;
        assert_parser_fail(parser, "\\u{DFFF}").await;
        assert_parser_fail(parser, "\\u{110000}").await;
    });
}

#[test]
fn escaped_char_continuous() {
    block_on(async {
        let parser = &mut super::escaped_char_continuous('\'').complete();
        assert_parser(parser, "a", Some('a')).await;
        assert_parser(parser, "\n", Some('\n')).await;
        assert_parser(parser, "\\n", Some('\n')).await;
        assert_parser(parser, "\\r", Some('\r')).await;
        assert_parser(parser, "\\t", Some('\t')).await;
        assert_parser(parser, "\\\\", Some('\\')).await;
        assert_parser(parser, "\\'", Some('\'')).await;
        assert_parser(parser, "\\x00", Some('\x00')).await;
        assert_parser(parser, "\\x0A", Some('\x0A')).await;
        assert_parser(parser, "\\x7f", Some('\x7f')).await;
        assert_parser(parser, "\\u{Ab}", Some('\u{AB}')).await;
        assert_parser(parser, "\\u{00a0}", Some('\u{A0}')).await;
        assert_parser(parser, "\\u{D7FF}", Some('\u{D7FF}')).await;
        assert_parser(parser, "\\u{E000}", Some('\u{E000}')).await;
        assert_parser(parser, "\\u{10FFFF}", Some('\u{10FFFF}')).await;
        assert_parser(parser, "\\\n", None).await;
        assert_parser_fail(parser, "'").await;
        assert_parser_fail(parser, "\\").await;
        assert_parser_fail(parser, "\\x80").await;
        assert_parser_fail(parser, "\\U{00}").await;
        assert_parser_fail(parser, "\\u{D800}").await;
        assert_parser_fail(parser, "\\u{DFFF}").await;
        assert_parser_fail(parser, "\\u{110000}").await;
    });
}
