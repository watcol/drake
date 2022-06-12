pub mod number;

use somen::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(u64),
    Float(f64),
}

pub fn literal<'a, I>() -> impl Parser<I, Output = Literal> + 'a
where
    I: Input<Ok = char> + 'a,
{
    choice((
        number::float().map(Literal::Float),
        number::integer().map(Literal::Integer),
    ))
}
