#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::{assert_parser, assert_parser_fail};

use somen::prelude::*;
use somen_language::numeric::integer::{integer_fixed, integer_trailing_zeros};

use crate::space::newline;

pub fn escaped_char<'a, I>(end: char) -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        newline(),
        none_of([end, '\\']),
        token('\\').prefix(choice((
            token('n').map(|_| '\n'),
            token('r').map(|_| '\r'),
            token('t').map(|_| '\t'),
            token('\\').map(|_| '\\'),
            token(end).map(move |_| end),
            token('x')
                .prefix(integer_fixed(2, 16, false))
                .try_map(|b: u8| {
                    if b <= 0x7F {
                        Ok(b as char)
                    } else {
                        Err("\\x00 to \\x7F")
                    }
                }),
            integer_trailing_zeros(16, false)
                .try_map(|u: u32| char::from_u32(u).ok_or("valid unicode codepoint"))
                .between(tag("u{"), token('}')),
        ))),
    ))
}

pub fn escaped_char_continuous<'a, I>(end: char) -> impl Parser<I, Output = Option<char>> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        newline().map(Some),
        none_of([end, '\\']).map(Some),
        token('\\').prefix(choice((
            token('n').map(|_| Some('\n')),
            token('r').map(|_| Some('\r')),
            token('t').map(|_| Some('\t')),
            token('\\').map(|_| Some('\\')),
            token(end).map(move |_| Some(end)),
            token('x')
                .prefix(integer_fixed(2, 16, false))
                .try_map(|b: u8| {
                    if b <= 0x7F {
                        Ok(Some(b as char))
                    } else {
                        Err("\\x00 to \\x7F")
                    }
                }),
            integer_trailing_zeros(16, false)
                .try_map(|u: u32| char::from_u32(u).ok_or("valid unicode codepoint").map(Some))
                .between(tag("u{"), token('}')),
            newline().map(|_| None),
        ))),
    ))
}
