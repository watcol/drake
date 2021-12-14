use super::utils::sign;
use pom::parser::*;

#[allow(dead_code)]
pub fn integer<'a>() -> Parser<'a, char, i64> {
    let digit = (one_of("123456789") + one_of("0123456789").repeat(0..))
        .collect()
        .convert(|s| s.iter().collect::<String>().parse::<i128>())
        | sym('0').map(|_| 0);
    let hex = tag("0x")
        * one_of("0123456789abcdefABCDEF")
            .repeat(1..)
            .convert(|s| i128::from_str_radix(&s.into_iter().collect::<String>(), 16));
    let oct = tag("0o")
        * one_of("01234567")
            .repeat(1..)
            .convert(|s| i128::from_str_radix(&s.into_iter().collect::<String>(), 8));
    let bin = tag("0b")
        * one_of("01")
            .repeat(1..)
            .convert(|s| i128::from_str_radix(&s.into_iter().collect::<String>(), 2));

    (sign() + (hex | oct | bin | digit)).convert(|(s, i)| i64::try_from(s * i))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_parser;

    #[test]
    fn digit() {
        test_parser!(integer, "0", 0); // Zero
        test_parser!(integer, "5", 5); // Normal one-digit number
        test_parser!(integer, "103", 103); // Normal number
        test_parser!(integer, "+42", 42); // Positive number
        test_parser!(integer, "-1023456789", -1023456789); // Negative number
        test_parser!(integer, "9223372036854775807", i64::MAX); // Maximum number.
        test_parser!(integer, "-9223372036854775808", i64::MIN); // Minimum number.
    }

    #[test]
    fn hex() {
        test_parser!(integer, "0x12", 18); // Normal number
        test_parser!(integer, "+0x12345abcdef", 0x12345abcdef); // Positive number
        test_parser!(integer, "-0x67890ABCDEF", -0x67890ABCDEF); // Negative number
        test_parser!(integer, "0x7fffffffffffffff", i64::MAX); // Maximum number.
        test_parser!(integer, "-0x8000000000000000", i64::MIN); // Minimum number.
    }

    #[test]
    fn oct() {
        test_parser!(integer, "0o12", 10); // Normal number
        test_parser!(integer, "+0o1234", 0o1234); // Positive number
        test_parser!(integer, "-0o5670", -0o5670); // Negative number
        test_parser!(integer, "0o777777777777777777777", i64::MAX); // Maximum number.
        test_parser!(integer, "-0o1000000000000000000000", i64::MIN); // Minimum number.
    }

    #[test]
    fn bin() {
        test_parser!(integer, "0b10110", 22); // Normal number
        test_parser!(integer, "+0b101", 5); // Positive number
        test_parser!(integer, "-0b110", -6); // Negative number
        test_parser!(
            integer,
            "0b111111111111111111111111111111111111111111111111111111111111111",
            i64::MAX
        ); // Maximum number.
        test_parser!(
            integer,
            "-0b1000000000000000000000000000000000000000000000000000000000000000",
            i64::MIN
        ); // Minimum number.
    }
}
