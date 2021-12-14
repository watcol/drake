use pom::parser::*;

pub fn sign<'a>() -> Parser<'a, char, i128> {
    let pos = sym('+').opt().map(|_| 1);
    let neg = sym('-').map(|_| -1);
    neg | pos
}

#[cfg(test)]
#[macro_export]
macro_rules! test_parser {
    ($parser:expr, $input:expr, $value:expr) => {
        let input: Vec<char> = $input.chars().collect();
        assert_eq!($parser().parse(&input), Ok($value));
    };
}
