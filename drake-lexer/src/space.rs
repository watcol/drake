#[cfg(test)]
mod tests;

use somen::error::Expects;
use somen::prelude::*;

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

pub fn continuous<'a, I>() -> impl IterableParser<I, Item = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    (token('\\'), whitespaces()).prefix(
        choice((newline().map(|_| None), comment().map(Some)))
            .repeat(1..)
            .flatten(),
    )
}

pub fn newline<'a, I>() -> impl Parser<I, Output = ()> + 'a
where
    I: Input<Ok = char> + 'a,
{
    one_of("\n\r")
        .prefix(one_of(" \t\n\r").repeat(..).discard())
        .expect("newline")
}

pub fn comment<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('#')
        .prefix(none_of("\n\r").repeat(..).collect())
        .expect("comment")
}
