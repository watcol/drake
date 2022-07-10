use alloc::vec::Vec;
use drake_types::ast::Statement;
use drake_types::token::Token;
use futures_util::TryStreamExt;
use somen::prelude::*;

pub async fn tokenize(source: &str) -> Result<Vec<Token>, ParseError> {
    let mut input = stream::from_iter(source.chars()).buffered_rewind();
    let mut lexer = drake_lexer::token().repeat(..);

    Ok(lexer.parse_iterable(&mut input).try_collect().await?)
}

pub async fn parse(tokens: &[Token]) -> Result<Vec<Statement<usize>>, ParseError> {
    let mut input = stream::from_slice(tokens);
    let mut parser = drake_parser::statement::statement().repeat(..);

    Ok(parser.parse_iterable(&mut input).try_collect().await?)
}

/// An error occured while parsing or tokenizing.
pub enum ParseError {
    /// A tokenizing error
    Tokenize(somen::error::Error<usize>),
    /// A parsing error
    Parse(somen::error::Error<usize>),
    /// An unexpected error (probably an internal bug)
    Unexpected,
}

type OriginalTokenizeError = somen::error::ParseError<
    usize,
    somen::stream::rewind::BufferedError<core::convert::Infallible>,
>;

type OriginalParseError = somen::error::ParseError<usize, core::convert::Infallible>;

impl From<OriginalTokenizeError> for ParseError {
    #[inline]
    fn from(err: OriginalTokenizeError) -> Self {
        match err {
            somen::error::ParseError::Parser(e) => Self::Tokenize(e),
            _ => Self::Unexpected,
        }
    }
}

impl From<OriginalParseError> for ParseError {
    #[inline]
    fn from(err: OriginalParseError) -> Self {
        match err {
            somen::error::ParseError::Parser(e) => Self::Parse(e),
            _ => Self::Unexpected,
        }
    }
}
