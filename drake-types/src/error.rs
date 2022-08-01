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

impl<L, E> From<ParseError<L, E>> for Error<L> {
    fn from(err: ParseError<L, E>) -> Self {
        match err {
            ParseError::Parser(err) => Error::ParseError {
                expects: err.expects,
                span: err.position,
            },
            ParseError::Stream(_) => Error::Unexpected,
        }
    }
}
