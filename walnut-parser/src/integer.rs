use super::utils::digits;
use pom::parser::*;

#[allow(dead_code)]
pub fn integer<'a>() -> Parser<'a, char, u64> {
    let dec = digits().collect().convert(|s| {
        s.iter()
            .collect::<String>()
            .parse()
            .map_err(|_| "Integer too big.")
    });
    let hex = tag("0x")
        * one_of("0123456789abcdefABCDEF").repeat(1..).convert(|h| {
            u64::from_str_radix(&h.into_iter().collect::<String>(), 16)
                .map_err(|_| "Integer too big.")
        });
    let oct = tag("0o")
        * one_of("01234567").repeat(1..).convert(|o| {
            u64::from_str_radix(&o.into_iter().collect::<String>(), 8)
                .map_err(|_| "Integer too big.")
        });
    let bin = tag("0b")
        * one_of("01").repeat(1..).convert(|b| {
            u64::from_str_radix(&b.into_iter().collect::<String>(), 2)
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
        assert_parser!(integer, "0", 0);
        should_fail_parser!(integer, "0042");
        assert_parser!(integer, "5", 5);
        assert_parser!(integer, "1234567890", 1234567890);

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
