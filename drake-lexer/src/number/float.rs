#[cfg(test)]
mod tests;

use somen::prelude::*;
use somen_language::numeric::{float::compute_float, integer::fold_digits, signed};

use super::{digits, digits_trailing_zeros};

pub fn float<'a, I>() -> impl Parser<I, Output = f64> + 'a
where
    I: Input<Ok = char> + 'a,
{
    let mantissa = fold_digits(digits(10), 0u64, 10, false).then(|(int, _, overflowed)| {
        if overflowed {
            value((int, 0, true)).left()
        } else {
            token('.')
                .prefix(fold_digits(digits_trailing_zeros(10), int, 10, false))
                .or(value((int, 0, false)))
                .right()
        }
    });

    let exponent = one_of("eE")
        .prefix(signed(
            |neg| fold_digits(digits_trailing_zeros(10), 0i32, 10, neg),
            true,
        ))
        .or(value((0, 0, false)));

    (mantissa, exponent).map(
        |((mantissa, count, man_overflowed), (exp, _, exp_overflowed))| {
            let (man, exp10) = (
                if man_overflowed { u64::MAX } else { mantissa },
                if exp_overflowed {
                    if exp < 0 {
                        i32::MIN
                    } else {
                        i32::MAX
                    }
                } else {
                    exp.saturating_sub(count as i32)
                },
            );
            compute_float(false, man, exp10).unwrap_or_else(|| man as f64 * 10f64.powi(exp10))
        },
    )
}
