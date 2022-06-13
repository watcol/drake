//! Whitespaces and comments
#[cfg(test)]
mod tests;

use alloc::string::String;
use core::ops::Range;
use somen::error::Expects;
use somen::prelude::*;

/// A parser for whitespaces
pub fn whitespaces<'a, I>() -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = char> + 'a,
{
    one_of(" \t")
        .expect(Expects::from_iter(["space", "tab"]))
        .repeat(..)
        .discard()
        .expect("whitespaces")
}

/// An iterable parser for continuous lines with comments
pub fn continuous<'a, I>() -> impl IterableParser<I, Item = (String, Range<usize>)> + 'a
where
    I: Input<Ok = char, Locator = usize> + 'a,
{
    (token('\\'), whitespaces()).prefix(
        choice((newline().map(|_| None), comment().with_position().map(Some)))
            .repeat(1..)
            .flatten(),
    )
}

/// A parser for newlines
pub fn newline<'a, I>() -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = char> + 'a,
{
    one_of("\n\r")
        .prefix(one_of(" \t\n\r").repeat(..).discard())
        .expect("newline")
}

/// A parser for comments
pub fn comment<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('#')
        .prefix(none_of("\n\r").repeat(..).collect())
        .expect("comment")
}
