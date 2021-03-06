#[cfg(test)]
mod tests;

use super::{digits, digits_trailing_zeros};
use drake_types::token::Radix;
use somen::prelude::*;
use somen_language::numeric::integer::fold_digits;

/// A parser for integers
pub fn integer<'a, I>() -> impl Parser<I, Output = (u64, Radix)> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        tag("0b").prefix(fold_digits(digits_trailing_zeros(2), 0, 2, false).try_map(
            |(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok((res, Radix::Binary))
                }
            },
        )),
        tag("0o").prefix(fold_digits(digits_trailing_zeros(8), 0, 8, false).try_map(
            |(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok((res, Radix::Octal))
                }
            },
        )),
        tag("0x").prefix(
            fold_digits(digits_trailing_zeros(16), 0, 16, false).try_map(|(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok((res, Radix::Hexadecimal))
                }
            }),
        ),
        fold_digits(digits(10), 0, 10, false).try_map(|(res, _, overflowed)| {
            if overflowed {
                Err("not too large number")
            } else {
                Ok((res, Radix::Decimal))
            }
        }),
    ))
}
