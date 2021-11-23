use std::ops::Range;

pub use lexer::tokens as lex;

#[derive(Clone, Debug, PartialEq)]
pub struct PosToken {
    file_id: usize,
    pos: Range<usize>,
    token: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Symbol(Symbol),
    Ident(String),
    Char(char),
    Str(String),
    Int(i64),
    Float(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    Newline,
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Remains,
    Exponent,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Not,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    UnderLine,
    At,
}

peg::parser! { grammar lexer() for str {
    pub rule tokens(file_id: usize) -> Vec<PosToken>
        = __? s:(token(file_id) ** _) __? { s }

    rule token(file_id: usize) -> PosToken
        = s:position!()
          t:(
            f:float() { Token::Float(f) }
            / i:integer() { Token::Int(i) }
            / s:string() { Token::Str(s) }
            / c:character() { Token::Char(c) }
            / i:ident() { Token::Ident(i) }
            / s:symbol() { Token::Symbol(s) }
          )
          e:position!() { PosToken{ file_id, pos: s..e, token: t } }

    rule symbol() -> Symbol =
        __ &[_] { Symbol::Newline }
        / "**" { Symbol::Exponent }
        / "==" { Symbol::Equals }
        / "!=" { Symbol::NotEquals }
        / "<=" { Symbol::LessThanEquals }
        / ">=" { Symbol::GreaterThanEquals }
        / "<<" { Symbol::LeftShift }
        / ">>" { Symbol::RightShift }
        / "=" { Symbol::Assign }
        / "+" { Symbol::Plus }
        / "-" { Symbol::Minus }
        / "*" { Symbol::Multiply }
        / "/" { Symbol::Divide }
        / "%" { Symbol::Remains }
        / "<" { Symbol::LessThan }
        / ">" { Symbol::GreaterThan }
        / "!" { Symbol::Not }
        / "&" { Symbol::And }
        / "|" { Symbol::Or }
        / "^" { Symbol::Xor }
        / "(" { Symbol::LeftParenthesis }
        / ")" { Symbol::RightParenthesis }
        / "{" { Symbol::LeftBrace }
        / "}" { Symbol::RightBrace }
        / "[" { Symbol::LeftBracket }
        / "]" { Symbol::RightBracket }
        / "," { Symbol::Comma }
        / "." { Symbol::Dot }
        / ":" { Symbol::Colon }
        / "_" { Symbol::UnderLine }
        / "@" { Symbol::At }
        / expected!("symbols")

    rule ident() -> String
        = ident_bare()
        / ident_raw()
        / expected!("identifier")
    rule ident_bare() -> String
        = s:$(['a'..='z'|'A'..='Z'] ['a'..='z'|'A'..='Z'|'0'..='9'|'_']*) {
            s.to_string()
        }
    rule ident_raw() -> String
        = "${" s:((
            c:$([^ '\\'|'}'|'\n'|'\r']) {?
                c.chars().next().map(|c| Some(c)).ok_or("char")
            }
          / normal_newline() { Some('\n') }
          / c:escape("}") { Some(c) }
          / "\\" normal_newline() { None }
        )*) "}" { s.into_iter().flat_map(|x| x).collect() }

    rule character() -> char
        = "'" c:(
            c:$([^ '\\'|'\''|'\n'|'\r']) {? c.chars().next().ok_or("char") }
          / escape("\'")
          ) "'" { c }

    rule string() -> String
        = raw_string()
        / normal_string()
        / expected!("string")

    rule raw_string() -> String
        = i:(e:$("\""*<3,>) { e.len() })
          s:((!("\""*<{i}>) c:(
                c:$([^ '\n'|'\r']) {? c.chars().next().ok_or("char") }
              / normal_newline() { '\n' }) { c })*)
          "\""*<{i}> {
              s.into_iter().collect()
          }

    rule normal_string() -> String
        = "\"" s:((
            c:$([^ '\\'|'"'|'\n'|'\r']) {?
                c.chars().next().map(|c| Some(c)).ok_or("char")
            }
          / normal_newline() { Some('\n') }
          / c:escape("\"") { Some(c) }
          / "\\" normal_newline() { None }
        )*) "\"" { s.into_iter().flat_map(|x| x).collect() }

    use peg::ParseLiteral;
    rule escape(lit: &'static str) -> char = "\\" s:(
        "n" { '\n' }
        / "r" { '\r' }
        / "t" { '\t' }
        / "\\" { '\\' }
        / ##parse_string_literal(lit) {? lit.chars().next().ok_or("literal") }
        / "x" h:$(['0'..='9'|'a'..='f'|'A'..='F']*<2>) {?
            u8::from_str_radix(h, 16).map(|h| h as char).or(Err("hex"))
        }
        / "u{" u:$(['0'..='9'|'a'..='f'|'A'..='F']*<2,8>) "}" {?
            u32::from_str_radix(u, 16)
                .or(Err("hex"))
                .and_then(|u| u.try_into().or(Err("unicode")))
        }
        / expected!("n, r, t, \\, newline, xXX, or u{XXXX}.")
    ) { s }

    rule normal_newline()
        = ("\r\n" / "\n" / "\r")

    rule integer() -> i64
        = s:sign() u:(hex() / oct() / bin() / dec()) { s * u }
    rule hex() -> i64
        = "0x" h:$(['0'..='9'|'a'..='f'|'A'..='F']++("_"*)) {?
            i64::from_str_radix(&h.replace('_', ""), 16).or(Err("too large"))
        }
    rule oct() -> i64
        = "0o" o:$(['0'..='7']++("_"*)) {?
            i64::from_str_radix(&o.replace('_', ""), 8).or(Err("too large"))
        }
    rule bin() -> i64
        = "0b" b:$(['0'|'1']++("_"*)) {?
            i64::from_str_radix(&b.replace('_', ""), 2).or(Err("too large"))
        }

    rule float() -> f64
        = s:$(sign() dec() f:frac()? e:exp()? {?
            if f.is_some() || e.is_some() {
                Ok(())
            } else {
                Err("invalid float")
            } }) {? s.replace('_', "").parse().or(Err("too large or small"))
        }
    rule frac() -> ()
        = "." $(['0'..='9']++("_"*))
    rule exp() -> ()
        = ("e"/"E") sign() $(['0'..='9'] ++ ("_"*))

    rule sign() -> i64
        = "+" { 1 } / "-" { -1 } / { 1 }
    rule dec() -> i64
        = d:$(['1'..='9'] "_"* ['0'..='9'] ** ("_"*)) {?
            i64::from_str_radix(&d.replace('_', ""), 10).or(Err("too large"))
        }
        / "0" { 0 }

    rule comment() = "#" [^ '\n'|'\r']*
    rule continuous() = "\\" [' '|'\t']* __
    rule _ = ([' '|'\t'] / continuous())*
    rule __ = _ comment()? ['\n'|'\r'] (
        [' '|'\t'|'\n'|'\r']
      / comment()
      / continuous()
    )*
}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace() {
        let code = indoc::indoc! {"
            # Comment
            'a'
            \r \t  \\ # Comment
            'b'

            # Comment
        "};
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 10..13,
                    token: Token::Char('a'),
                },
                PosToken {
                    file_id: 0,
                    pos: 13..31,
                    token: Token::Symbol(Symbol::Newline),
                },
                PosToken {
                    file_id: 0,
                    pos: 31..34,
                    token: Token::Char('b'),
                },
            ])
        );
    }

    #[test]
    fn symbols() {
        let code = ". _ = == ! !=";
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..1,
                    token: Token::Symbol(Symbol::Dot)
                },
                PosToken {
                    file_id: 0,
                    pos: 2..3,
                    token: Token::Symbol(Symbol::UnderLine)
                },
                PosToken {
                    file_id: 0,
                    pos: 4..5,
                    token: Token::Symbol(Symbol::Assign),
                },
                PosToken {
                    file_id: 0,
                    pos: 6..8,
                    token: Token::Symbol(Symbol::Equals),
                },
                PosToken {
                    file_id: 0,
                    pos: 9..10,
                    token: Token::Symbol(Symbol::Not),
                },
                PosToken {
                    file_id: 0,
                    pos: 11..13,
                    token: Token::Symbol(Symbol::NotEquals),
                },
            ])
        );
    }

    #[test]
    fn idents() {
        let code = indoc::indoc! {r#"
            f00_B4r ${\\{All\u{00A0}characters\
            \ncan be used.\}}
        "#};
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..7,
                    token: Token::Ident(String::from("f00_B4r")),
                },
                PosToken {
                    file_id: 0,
                    pos: 8..53,
                    token: Token::Ident(String::from("\\{All\u{A0}characters\ncan be used.}")),
                }
            ])
        )
    }

    #[test]
    fn chars() {
        let code = "'a' '\\n' '\\'' '\\\\'";
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..3,
                    token: Token::Char('a'),
                },
                PosToken {
                    file_id: 0,
                    pos: 4..8,
                    token: Token::Char('\n'),
                },
                PosToken {
                    file_id: 0,
                    pos: 9..13,
                    token: Token::Char('\''),
                },
                PosToken {
                    file_id: 0,
                    pos: 14..18,
                    token: Token::Char('\\'),
                },
            ])
        )
    }

    #[test]
    fn string() {
        let code1 = indoc::indoc! {r#"
            "" "\t\r\x00
            \
            \"foo\""
        "#};
        assert_eq!(
            lex(code1, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..2,
                    token: Token::Str(String::from(""))
                },
                PosToken {
                    file_id: 0,
                    pos: 3..23,
                    token: Token::Str(String::from("\t\r\x00\n\"foo\""))
                }
            ])
        );
        let code2 = "\"\"\"\\not\\escaped\n\r\n\r\\\\\"\"\"";
        assert_eq!(
            lex(code2, 0),
            Ok(vec![PosToken {
                file_id: 0,
                pos: 0..24,
                token: Token::Str(String::from("\\not\\escaped\n\n\n\\\\"))
            }])
        );
        let code3 = r#"""""(""")"""""#;
        assert_eq!(
            lex(code3, 0),
            Ok(vec![PosToken {
                file_id: 0,
                pos: 0..13,
                token: Token::Str(String::from(r#"(""")"#)),
            }])
        );
    }

    #[test]
    fn ints() {
        let code = "0xd34db33f +0o644 -0b10011110 42 -1_2__3";
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..10,
                    token: Token::Int(0xd34db33f),
                },
                PosToken {
                    file_id: 0,
                    pos: 11..17,
                    token: Token::Int(0o644),
                },
                PosToken {
                    file_id: 0,
                    pos: 18..29,
                    token: Token::Int(-0b10011110),
                },
                PosToken {
                    file_id: 0,
                    pos: 30..32,
                    token: Token::Int(42),
                },
                PosToken {
                    file_id: 0,
                    pos: 33..40,
                    token: Token::Int(-1_2__3),
                },
            ])
        );
    }

    #[test]
    fn floats() {
        let code = "1_2_3.0_2_3 +23e-0_2 -1.1e+2 2e2";
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 0..11,
                    token: Token::Float(123.023f64),
                },
                PosToken {
                    file_id: 0,
                    pos: 12..20,
                    token: Token::Float(23e-2f64),
                },
                PosToken {
                    file_id: 0,
                    pos: 21..28,
                    token: Token::Float(-1.1e2f64),
                },
                PosToken {
                    file_id: 0,
                    pos: 29..32,
                    token: Token::Float(2e2f64),
                },
            ])
        );
    }
}
