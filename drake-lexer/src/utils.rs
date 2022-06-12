use somen::prelude::*;
use somen_language::numeric::integer::{integer_fixed, integer_trailing_zeros};

#[cfg(test)]
pub use test_utils::{assert_parser, assert_parser_fail};

pub fn escaped_char<'a, I>(end: char) -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    none_of([end, '\\']).or(token('\\').prefix(choice((
        token('n').map(|_| '\n'),
        token('r').map(|_| '\r'),
        token('t').map(|_| '\t'),
        token('\\').map(|_| '\\'),
        token(end).map(move |_| end),
        token('x')
            .prefix(integer_fixed(2, 16, false))
            .try_map(|b: u8| {
                if b <= 0x7F {
                    Ok(b as char)
                } else {
                    Err("\\x00 to \\x7F")
                }
            }),
        integer_trailing_zeros(16, false)
            .try_map(|u: u32| char::from_u32(u).ok_or("valid unicode codepoint"))
            .between(tag("u{"), token('}')),
    ))))
}

#[cfg(test)]
mod tests {
    use super::{assert_parser, assert_parser_fail};
    use futures_executor::block_on;
    use somen::prelude::*;

    #[test]
    fn escaped_char() {
        block_on(async {
            let parser = &mut super::escaped_char('\'').complete();
            assert_parser(parser, "a", 'a').await;
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
}

#[cfg(test)]
mod test_utils {
    use core::fmt::Debug;
    use somen::prelude::*;
    use somen::stream::rewind::BufferedRewinder;
    use somen::stream::{InfallibleStream, IteratorStream};
    use std::str::Chars;

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
}
