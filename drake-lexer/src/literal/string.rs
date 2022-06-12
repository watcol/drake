#[cfg(test)]
mod tests;

use somen::error::Expects;
use somen::prelude::*;

use crate::utils::{escaped_char, escaped_char_continuous, newline};

pub fn character<'a, I>() -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char('\'')
        .between(token('\''), token('\''))
        .expect("character")
}

pub fn string<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char_continuous('"')
        .expect(Expects::from_iter(["character", "continuous line"]))
        .repeat(..)
        .flatten()
        .collect()
        .between(token('"'), token('"'))
        .expect("string")
}

pub fn raw_string<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('"')
        .repeat(3..)
        .count()
        .then(|n| {
            token('"')
                .times(n)
                .discard()
                .fail()
                .prefix(newline().or(any()))
                .expect("raw character")
                .repeat(..)
                .skip(token('"').times(n).discard())
        })
        .collect()
        .expect("raw string")
}
