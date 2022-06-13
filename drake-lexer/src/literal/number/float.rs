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

    let exponent = one_of("eE").prefix(signed(
        |neg| fold_digits(digits_trailing_zeros(10), 0i32, 10, neg),
        true,
    ));

    (mantissa, exponent.opt())
        .try_map(|((mantissa, count, man_overflowed), opt)| {
            let man = if man_overflowed { u64::MAX } else { mantissa };
            let exp10 = match opt {
                Some((exp, _, true)) if exp < 0 => i32::MIN,
                Some((_, _, true)) => i32::MAX,
                Some((exp, _, false)) => exp.saturating_sub(count as i32),
                None if count == 0 => return Err(""),
                None => -(count as i32),
            };

            Ok(compute_float(false, man, exp10).unwrap_or_else(|| man as f64 * 10f64.powi(exp10)))
        })
        .rewindable()
        .expect("float")
}
