//! Literal values
#[cfg(test)]
mod tests;

pub mod number;
pub mod string;

use drake_types::token::Literal;
use somen::prelude::*;

/// A parser for literals
pub fn literal<'a, I>() -> impl Parser<I, Output = Literal> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        number::float().map(Literal::Float),
        number::integer().map(|(i, radix)| Literal::Integer(i, radix)),
        string::character().map(Literal::Character),
        string::string().map(|(s, kind)| Literal::String(s, kind)),
    ))
    .expect("literal")
}
