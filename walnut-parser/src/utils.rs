use pom::parser::*;

pub fn digits<'a>(radix: u32) -> Parser<'a, char, String> {
    let underscore = || sym('_').repeat(0..);
    let digit = || is_a(move |c: char| c.is_digit(radix));
    (digit() + (underscore() * digit()).repeat(0..))
        .map(|(first, rest)| std::iter::once(first).chain(rest.into_iter()).collect())
}

pub fn digits_no_leading_zero<'a>(radix: u32) -> Parser<'a, char, String> {
    let underscore = || sym('_').repeat(0..);
    let digit = || is_a(move |c: char| c.is_digit(radix));
    let non_zero = is_a(move |c: char| c.is_digit(radix) && c != '0');
    (non_zero + (underscore() * digit()).repeat(0..))
        .map(|(first, rest)| std::iter::once(first).chain(rest.into_iter()).collect())
        | (sym('0') + !digit()).map(|_| String::from("0"))
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_parser {
    ($parser:expr, $input:expr, $value:expr) => {
        let input: Vec<char> = $input.chars().collect();
        assert_eq!($parser().parse(&input), Ok($value));
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_fail_parser {
    ($parser:expr, $input:expr) => {
        let input: Vec<char> = $input.chars().collect();
        assert!($parser().parse(&input).is_err());
    };
}
