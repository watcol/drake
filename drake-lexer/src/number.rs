#[cfg(test)]
mod tests;

use somen::prelude::*;
use somen_language::numeric::{digit, integer::fold_digits, non_zero_digit};

pub fn integer<'a, I>() -> impl Parser<I, Output = u64> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        fold_digits(digits(10), 0, 10, false).try_map(|(res, _, overflowed)| {
            if overflowed {
                Err("not too large number")
            } else {
                Ok(res)
            }
        }),
        tag("0x").prefix(
            fold_digits(digits_trailing_zeros(16), 0, 16, false).try_map(|(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok(res)
                }
            }),
        ),
        tag("0o").prefix(fold_digits(digits_trailing_zeros(8), 0, 8, false).try_map(
            |(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok(res)
                }
            },
        )),
        tag("0b").prefix(fold_digits(digits_trailing_zeros(2), 0, 2, false).try_map(
            |(res, _, overflowed)| {
                if overflowed {
                    Err("not too large number")
                } else {
                    Ok(res)
                }
            },
        )),
    ))
}

fn digits<'a, I>(radix: u8) -> impl IterableParser<I, Item = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    non_zero_digit(radix).once().chain(
        choice((digit(radix).map(Some), token('_').map(|_| None)))
            .repeat(..)
            .flatten(),
    )
}

fn digits_trailing_zeros<'a, I>(radix: u8) -> impl IterableParser<I, Item = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    digit(radix).once().chain(
        digit(radix)
            .map(Some)
            .or(token('_').map(|_| None))
            .repeat(..)
            .flatten(),
    )
}
