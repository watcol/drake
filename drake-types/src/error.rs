use core::convert::Infallible;
use core::ops::Range;
use somen::error::{Expects, ParseError};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error<L> {
    ParseError {
        expects: Expects,
        span: Range<L>,
    },
    DuplicateKey {
        found: Range<L>,
        existing: Option<Range<L>>,
    },
    BuiltinNotFound {
        span: Range<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Range<L>,
    },
    Unexpected,
}

impl<L> From<ParseError<L, Infallible>> for Error<L> {
    fn from(err: ParseError<L, Infallible>) -> Self {
        match err {
            ParseError::Parser(err) => Error::ParseError {
                expects: err.expects,
                span: err.position,
            },
            _ => unreachable!(),
        }
    }
}

impl<L> From<ParseError<L, ParseError<L, Infallible>>> for Error<L> {
    fn from(err: ParseError<L, ParseError<L, Infallible>>) -> Self {
        match err {
            ParseError::Parser(err) | ParseError::Stream(ParseError::Parser(err)) => {
                Error::ParseError {
                    expects: err.expects,
                    span: err.position,
                }
            }
            _ => unreachable!(),
        }
    }
}
