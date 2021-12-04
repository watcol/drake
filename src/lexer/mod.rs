#[cfg(test)]
mod tests;

pub use lexer::tokens as lexer;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct PosToken {
    pos: Range<usize>,
    token: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Symbol(Symbol),
    Ident(String),
    Bool(bool),
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
}

peg::parser! { grammar lexer() for str {
    pub rule tokens() -> Vec<PosToken>
        = __? s:(token() ** _) __? { s }

    rule token() -> PosToken
        = s:position!()
          t:(
            f:float() { Token::Float(f) }
            / i:integer() { Token::Int(i) }
            / s:string() { Token::Str(s) }
            / c:character() { Token::Char(c) }
            / b:boolean() { Token::Bool(b) }
            / i:ident() { Token::Ident(i) }
            / s:symbol() { Token::Symbol(s) }
          )
          e:position!() { PosToken{ pos: s..e, token: t } }

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
                c.chars().next().map(Some).ok_or("char")
            }
          / normal_newline() { Some('\n') }
          / c:escape("}") { Some(c) }
          / "\\" normal_newline() { None }
        )*) "}" { s.into_iter().flatten().collect() }

    rule boolean() -> bool
        = "@true" { true }
        / "@false" { false }

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
                c.chars().next().map(Some).ok_or("char")
            }
          / normal_newline() { Some('\n') }
          / c:escape("\"") { Some(c) }
          / "\\" normal_newline() { None }
        )*) "\"" { s.into_iter().flatten().collect() }

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
        = hex() / oct() / bin() / dec()
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
        = s:$(dec() f:frac()? e:exp()? {?
            if f.is_some() || e.is_some() {
                Ok(())
            } else {
                Err("invalid float")
            } }) {? s.replace('_', "").parse().or(Err("too large or small"))
        }
        / "@inf" { f64::INFINITY }
        / "@nan" { f64::NAN }
    rule frac() -> ()
        = "." $(['0'..='9']++("_"*))
    rule exp() -> ()
        = ("e"/"E") ("+"/"-")? $(['0'..='9'] ++ ("_"*))

    rule dec() -> i64
        = d:$(['1'..='9'] "_"* ['0'..='9'] ** ("_"*)) {?
            d.replace('_', "").parse().or(Err("too large"))
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
