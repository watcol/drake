mod float;
mod integer;
#[cfg(test)]
mod tests;

pub use integer::integer;

use somen::prelude::*;
use somen_language::numeric::{digit, non_zero_digit};

fn digits<'a, I>(radix: u8) -> impl IterableParser<I, Item = char> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice_iterable((
        non_zero_digit(radix).once().chain(
            choice((digit(radix).map(Some), token('_').map(|_| None)))
                .repeat(..)
                .flatten(),
        ),
        token('0').once(),
    ))
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
