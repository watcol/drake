//! String literals
#[cfg(test)]
mod tests;

use alloc::string::String;
use drake_types::token::StringKind;
use somen::error::Expects;
use somen::prelude::*;

use crate::space::newline;
use crate::utils::{escaped_char, escaped_char_continuous};

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
pub fn string<'a, I>() -> impl Parser<I, Output = (String, StringKind)> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        raw_string().map(|(s, n)| (s, StringKind::Raw(n))),
        normal_string().map(|s| (s, StringKind::Normal)),
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
pub fn raw_string<'a, I>() -> impl Parser<I, Output = (String, u8)> + 'a
where
    I: Input<Ok = char> + 'a,
{
    token('"')
        .repeat(3..)
        .count()
        .spanned()
        .then(|n| {
            newline()
                .or(any())
                .expect("raw character")
                .until(token('"').times(n).discard().spanned())
                .collect()
                .map(move |s| (s, n as u8))
        })
        .expect("raw string")
}
