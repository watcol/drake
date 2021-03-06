use core::fmt::Debug;
use core::str::Chars;
use somen::prelude::*;
use somen::stream::rewind::BufferedRewinder;
use somen::stream::{InfallibleStream, IteratorStream};

pub async fn assert_parser<T: PartialEq + Debug>(
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

pub async fn assert_parser_fail(
    parser: &mut impl Parser<BufferedRewinder<InfallibleStream<IteratorStream<Chars<'static>>>>>,
    s: &'static str,
) {
    let mut stream = stream::from_iter(s.chars()).buffered_rewind();
    assert!(parser.parse(&mut stream).await.is_err());
}
