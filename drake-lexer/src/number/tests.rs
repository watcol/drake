use core::fmt::Debug;
use core::str::Chars;

use futures_executor::block_on;
use somen::prelude::*;
use somen::stream::rewind::BufferedRewinder;
use somen::stream::{InfallibleStream, IteratorStream};

#[test]
fn integer() {
    block_on(async {
        let parser = &mut super::integer().complete();
        assert_parser(parser, "42", 42).await;
        assert_parser(parser, "0xDEADBEEF", 0xDEADBEEF).await;
        assert_parser(parser, "0xcafebabe", 0xcafebabe).await;
        assert_parser(parser, "0o644", 0o644).await;
        assert_parser(parser, "0b01010110", 0b01010110).await;
    })
}

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

async fn assert_parser<T: PartialEq + Debug>(
    parser: &mut impl Parser<
        BufferedRewinder<InfallibleStream<IteratorStream<Chars<'static>>>>,
        Output = T,
    >,
    s: &'static str,
    res: T,
) {
    let mut stream = stream::from_iter(s.chars()).buffered_rewind();
    assert_eq!(parser.parse(&mut stream).await, Ok(res));
}

async fn assert_parser_fail(
    parser: &mut impl Parser<BufferedRewinder<InfallibleStream<IteratorStream<Chars<'static>>>>>,
    s: &'static str,
) {
    let mut stream = stream::from_iter(s.chars()).buffered_rewind();
    assert!(parser.parse(&mut stream).await.is_err());
}
