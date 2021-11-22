use std::ops::Range;

pub use lexer::tokens as lex;

peg::parser! { grammar lexer() for str {
    rule symbol() -> Symbol =
        "**" { Symbol::Exponent }
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

    rule comment() = "#" [^ '\n'|'\r']*
    rule _ = ([' '|'\t'] / ("\\" [' '|'\t']* __))*
    rule __ = comment()? ['\n'|'\r'] ([' '|'\t'|'\n'|'\r'] / comment())*

    rule token(file_id: usize) -> PosToken
        = s:position!()
          t:(sym:symbol() { Token::Symbol(sym) })
          e:position!() { PosToken{ file_id, pos: s..e, token: t } }

    rule statement(file_id: usize) -> Vec<PosToken>
        = _ ts:(token(file_id) ++ _) _ { ts }

    pub rule tokens(file_id: usize) -> Vec<Vec<PosToken>>
        = __? s:(statement(file_id) ** __) __? { s }
}}

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
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols() {
        let code = indoc::indoc! {"
            # Comment Line
            \t   \r = + - * / % ** == != < > <= >= \\ # After Comment

            ! & | ^ << >> ( ) { } [ ] , . : _ @
        "};
        assert_eq!(
            lex(code, 0),
            Ok(vec![vec![
                PosToken {
                    file_id: 0,
                    pos: 21..22,
                    token: Token::Symbol(Symbol::Assign)
                },
                PosToken {
                    file_id: 0,
                    pos: 23..24,
                    token: Token::Symbol(Symbol::Plus)
                },
                PosToken {
                    file_id: 0,
                    pos: 25..26,
                    token: Token::Symbol(Symbol::Minus)
                },
                PosToken {
                    file_id: 0,
                    pos: 27..28,
                    token: Token::Symbol(Symbol::Multiply)
                },
                PosToken {
                    file_id: 0,
                    pos: 29..30,
                    token: Token::Symbol(Symbol::Divide),
                },
                PosToken {
                    file_id: 0,
                    pos: 31..32,
                    token: Token::Symbol(Symbol::Remains),
                },
                PosToken {
                    file_id: 0,
                    pos: 33..35,
                    token: Token::Symbol(Symbol::Exponent),
                },
                PosToken {
                    file_id: 0,
                    pos: 36..38,
                    token: Token::Symbol(Symbol::Equals),
                },
                PosToken {
                    file_id: 0,
                    pos: 39..41,
                    token: Token::Symbol(Symbol::NotEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 42..43,
                    token: Token::Symbol(Symbol::LessThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 44..45,
                    token: Token::Symbol(Symbol::GreaterThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 46..48,
                    token: Token::Symbol(Symbol::LessThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 49..51,
                    token: Token::Symbol(Symbol::GreaterThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 71..72,
                    token: Token::Symbol(Symbol::Not),
                },
                PosToken {
                    file_id: 0,
                    pos: 73..74,
                    token: Token::Symbol(Symbol::And),
                },
                PosToken {
                    file_id: 0,
                    pos: 75..76,
                    token: Token::Symbol(Symbol::Or),
                },
                PosToken {
                    file_id: 0,
                    pos: 77..78,
                    token: Token::Symbol(Symbol::Xor),
                },
                PosToken {
                    file_id: 0,
                    pos: 79..81,
                    token: Token::Symbol(Symbol::LeftShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 82..84,
                    token: Token::Symbol(Symbol::RightShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 85..86,
                    token: Token::Symbol(Symbol::LeftParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 87..88,
                    token: Token::Symbol(Symbol::RightParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 89..90,
                    token: Token::Symbol(Symbol::LeftBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 91..92,
                    token: Token::Symbol(Symbol::RightBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 93..94,
                    token: Token::Symbol(Symbol::LeftBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 95..96,
                    token: Token::Symbol(Symbol::RightBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 97..98,
                    token: Token::Symbol(Symbol::Comma),
                },
                PosToken {
                    file_id: 0,
                    pos: 99..100,
                    token: Token::Symbol(Symbol::Dot),
                },
                PosToken {
                    file_id: 0,
                    pos: 101..102,
                    token: Token::Symbol(Symbol::Colon),
                },
                PosToken {
                    file_id: 0,
                    pos: 103..104,
                    token: Token::Symbol(Symbol::UnderLine),
                },
                PosToken {
                    file_id: 0,
                    pos: 105..106,
                    token: Token::Symbol(Symbol::At),
                },
            ]])
        );
    }
}
