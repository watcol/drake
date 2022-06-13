#[cfg(test)]
mod tests;

pub mod number;
pub mod string;

use somen::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(u64),
    Float(f64),
    Character(char),
    String(String),
}

pub fn literal<'a, I>() -> impl Parser<I, Output = Literal> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        number::float().map(Literal::Float),
        number::integer().map(Literal::Integer),
        string::character().map(Literal::Character),
        string::string().map(Literal::String),
    ))
}
