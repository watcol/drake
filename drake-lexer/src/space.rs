//! Whitespaces and comments
#[cfg(test)]
mod tests;

use alloc::string::String;
use somen::error::Expects;
use somen::prelude::*;

/// A parser for sequences of whitespaces
pub fn whitespaces<'a, I>() -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = char> + 'a,
{
    one_of(" \t")
        .expect(Expects::from_iter(["space", "tab"]))
        .repeat(1..)
        .discard()
        .expect("whitespaces")
}

/// A parser for newlines
pub fn newline<'a, I>() -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        token('\n'),
        tag("\r\n").map(|_| '\n'),
        token('\r').map(|_| '\n'),
    ))
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
