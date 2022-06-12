use somen::prelude::*;

use crate::utils::escaped_char;

pub fn character<'a, I>() -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    escaped_char('\'').between(token('\''), token('\''))
}

pub fn string<'a, I>() -> impl Parser<I, Output = String> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((newline(), escaped_char('"')))
        .repeat(..)
        .collect()
        .between(token('"'), token('"'))
}

#[inline]
pub fn newline<'a, I>() -> impl Parser<I, Output = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        token('\n'),
        tag("\r\n").map(|_| '\n'),
        token('\r').map(|_| '\n'),
    ))
}
