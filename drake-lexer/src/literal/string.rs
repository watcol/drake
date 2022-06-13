//! String literals
#[cfg(test)]
mod tests;

use alloc::string::String;
use somen::error::Expects;
use somen::prelude::*;

use crate::utils::{escaped_char, escaped_char_continuous, newline};

/// A parser for characters
pub fn character<'a, I>() -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char('\'')
        .between(token('\''), token('\''))
        .expect("character")
}

/// A parser for (normal and raw) strings
pub fn string<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        token('"').times(3).discard().fail().prefix(normal_string()),
        raw_string(),
    ))
}

/// A parser for normal strings
pub fn normal_string<'a, I>() -> impl Parser<I, Output = String> + 'a
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

/// A parser for raw strings
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
