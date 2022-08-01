use alloc::vec::Vec;
use core::convert::Infallible;
use core::ops::Range;
use core::pin::Pin;
use core::task::{Context, Poll};
use drake_types::ast::Statement;
use drake_types::error::Error;
use drake_types::token::Token as TokenKind;
use futures_util::{Stream, TryStreamExt};
use pin_project_lite::pin_project;
use somen::prelude::*;
use somen::stream::{Rewind, SliceStream};

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

pub async fn tokenize<T: AsRef<[u8]> + ?Sized>(source: &T) -> Result<Vec<Token>, Error<usize>> {
    let mut bytes = stream::from_slice(source.as_ref());
    let mut decoder = somen_decode::utf8().repeat(..).complete();
    let mut input = decoder.parse_iterable(&mut bytes);
    let mut lexer = drake_lexer::token()
        .with_position()
        .map(|(kind, span)| Token { kind, span })
        .repeat(..);

    Ok(lexer.parse_iterable(&mut input).try_collect().await?)
}

pub async fn parse(tokens: &[Token]) -> Result<Vec<Statement<usize>>, Error<usize>> {
    let mut input = TokenStream::from(tokens);
    let mut parser = drake_parser::statement::statement().repeat(..);

    Ok(parser.parse_iterable(&mut input).try_collect().await?)
}

pin_project! {
    struct TokenStream<'a> {
        #[pin]
        inner: SliceStream<'a, Token>,
        cur: usize,
    }
}

impl<'a> From<&'a [Token]> for TokenStream<'a> {
    #[inline]
    fn from(slice: &'a [Token]) -> Self {
        Self {
            inner: stream::from_slice(slice),
            cur: 0,
        }
    }
}

impl Stream for TokenStream<'_> {
    type Item = Result<TokenKind, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.inner.poll_next(cx) {
            Poll::Ready(Some(Ok(Token { kind, span }))) => {
                *this.cur = span.end;
                Poll::Ready(Some(Ok(kind)))
            }
            Poll::Ready(Some(Err(_))) => unreachable!(),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Positioned for TokenStream<'_> {
    type Locator = usize;

    #[inline]
    fn position(&self) -> Self::Locator {
        self.cur
    }
}

impl Rewind for TokenStream<'_> {
    type Marker = usize;

    #[inline]
    fn mark(self: Pin<&mut Self>) -> Result<Self::Marker, Self::Error> {
        self.project().inner.mark()
    }

    #[inline]
    fn rewind(self: Pin<&mut Self>, marker: Self::Marker) -> Result<(), Self::Error> {
        self.project().inner.rewind(marker)
    }

    #[inline]
    fn drop_marker(self: Pin<&mut Self>, marker: Self::Marker) -> Result<(), Self::Error> {
        self.project().inner.drop_marker(marker)
    }
}
