use super::utils::{digits, digits_no_leading_zero};
use pom::parser::*;

#[allow(dead_code)]
pub fn integer<'a>() -> Parser<'a, char, u64> {
    let dec = digits_no_leading_zero(10).convert(|s| s.parse().map_err(|_| "Integer too big."));
    let hex = tag("0x")
        * digits(16).convert(|s| u64::from_str_radix(&s, 16).map_err(|_| "Integer too big."));
    let oct = tag("0o")
        * digits(8).convert(|s| u64::from_str_radix(&s, 8).map_err(|_| "Integer too big."));
    let bin = tag("0b")
        * digits(2).convert(|s| u64::from_str_radix(&s, 2).map_err(|_| "Integer too big."));

    hex | oct | bin | dec
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_parser, should_fail_parser};

    #[test]
    fn dec() {
        assert_parser!(integer, "0", 0);
        should_fail_parser!(integer, "0042");
        assert_parser!(integer, "5", 5);
        assert_parser!(integer, "1234567890", 1234567890);
        assert_parser!(integer, "1_2__3_4", 1234);
        should_fail_parser!(integer, "_1");

        // Maximum number.
        assert_parser!(integer, "18446744073709551615", u64::MAX);
        should_fail_parser!(integer, "18446744073709551616");
    }

    #[test]
    fn hex() {
        assert_parser!(integer, "0x01234abcdef", 0x01234abcdef);
        assert_parser!(integer, "0x56789ABCDEF", 0x56789abcdef);
    }

    #[test]
    fn oct() {
        assert_parser!(integer, "0o0123", 0o0123);
        assert_parser!(integer, "0o4567", 0o4567);
    }

    #[test]
    fn bin() {
        assert_parser!(integer, "0b010110", 0b010110); // Normal number
    }
}
