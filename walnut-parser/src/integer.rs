use super::utils::{digits, sign};
use pom::parser::*;

#[allow(dead_code)]
pub fn integer<'a>() -> Parser<'a, char, i64> {
    let dec = (sign() - digits()).collect().convert(|s| {
        s.iter()
            .collect::<String>()
            .parse::<i64>()
            .map_err(|_| "Integer too big.")
    });
    let hex =
        (sign() + tag("0x") * one_of("0123456789abcdefABCDEF").repeat(1..)).convert(|(s, i)| {
            i64::from_str_radix(&s.into_iter().chain(i.into_iter()).collect::<String>(), 16)
                .map_err(|_| "Integer too big.")
        });
    let oct = (sign() + tag("0o") * one_of("01234567").repeat(1..)).convert(|(s, i)| {
        i64::from_str_radix(&s.into_iter().chain(i.into_iter()).collect::<String>(), 8)
            .map_err(|_| "Integer too big.")
    });
    let bin = (sign() + tag("0b") * one_of("01").repeat(1..)).convert(|(s, i)| {
        i64::from_str_radix(&s.into_iter().chain(i.into_iter()).collect::<String>(), 2)
            .map_err(|_| "Integer too big.")
    });

    hex | oct | bin | dec
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_parser, should_fail_parser};

    #[test]
    fn dec() {
        assert_parser!(integer, "0", 0); // Zero
        should_fail_parser!(integer, "012"); // Value starts with '0'
        assert_parser!(integer, "5", 5); // Normal one-digit number
        assert_parser!(integer, "103", 103); // Normal number
        assert_parser!(integer, "+42", 42); // Positive number
        assert_parser!(integer, "-1023456789", -1023456789); // Negative number

        // Maximum number.
        assert_parser!(integer, "9223372036854775807", i64::MAX);
        should_fail_parser!(integer, "9223372036854775808");

        // Minimum number.
        assert_parser!(integer, "-9223372036854775808", i64::MIN);
        should_fail_parser!(integer, "-9223372036854775809");
    }

    #[test]
    fn hex() {
        assert_parser!(integer, "0x12", 0x12); // Normal number
        assert_parser!(integer, "+0x01234abcdef", 0x01234abcdef); // Positive number
        assert_parser!(integer, "-0x56789ABCDEF", -0x56789ABCDEF); // Negative number
    }

    #[test]
    fn oct() {
        assert_parser!(integer, "0o12", 0o12); // Normal number
        assert_parser!(integer, "+0o0123", 0o0123); // Positive number
        assert_parser!(integer, "-0o4567", -0o4567); // Negative number
    }

    #[test]
    fn bin() {
        assert_parser!(integer, "0b10110", 0b10110); // Normal number
        assert_parser!(integer, "+0b101", 0b101); // Positive number
        assert_parser!(integer, "-0b010", -0b010); // Negative number
    }
}
