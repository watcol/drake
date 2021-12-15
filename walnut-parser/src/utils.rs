use pom::parser::*;

pub fn digits<'a>() -> Parser<'a, char, &'a [char]> {
    ((one_of("123456789") - one_of("0123456789").repeat(0..)) | (sym('0') - !one_of("0123456789")))
        .collect()
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
