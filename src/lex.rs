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

    rule boolean() -> bool = "true" { true } / "false" { false }

    rule comment() = "#" [^ '\n'|'\r']*
    rule continuous() = "\\" [' '|'\t']* __
    rule _ = ([' '|'\t'] / continuous())*
    rule __ = _ comment()? ['\n'|'\r'] ([' '|'\t'|'\n'|'\r'] / comment() / continuous())*

    rule token(file_id: usize) -> PosToken
        = s:position!()
          t:(s:symbol() { Token::Symbol(s) }
           / b:boolean() { Token::Bool(b) })
          e:position!() { PosToken{ file_id, pos: s..e, token: t } }

    rule statement(file_id: usize) -> Vec<PosToken>
        = ts:(token(file_id) ++ _) { ts }

    pub rule tokens(file_id: usize) -> Vec<Vec<PosToken>>
        = __? s:(statement(file_id) ** __) __? { s }
        / __? { Vec::new() }
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
    fn whitespace() {
        let code = indoc::indoc! {"
            # Comment
            \r \t  \\ # Comment

            # Comment
        "};
        assert_eq!(lex(code, 0), Ok(Vec::new()));
    }

    #[test]
    fn bools() {
        let code = "true false";
        assert_eq!(
            lex(code, 0),
            Ok(vec![vec![
                PosToken {
                    file_id: 0,
                    pos: 0..4,
                    token: Token::Bool(true),
                },
                PosToken {
                    file_id: 0,
                    pos: 5..10,
                    token: Token::Bool(false),
                },
            ]])
        )
    }

    #[test]
    fn symbols() {
        let code = "= + - * / % ** == != < > <= >= ! & | ^ << >> ( ) { } [ ] , . : _ @";
        assert_eq!(
            lex(code, 0),
            Ok(vec![vec![
                PosToken {
                    file_id: 0,
                    pos: 0..1,
                    token: Token::Symbol(Symbol::Assign)
                },
                PosToken {
                    file_id: 0,
                    pos: 2..3,
                    token: Token::Symbol(Symbol::Plus)
                },
                PosToken {
                    file_id: 0,
                    pos: 4..5,
                    token: Token::Symbol(Symbol::Minus)
                },
                PosToken {
                    file_id: 0,
                    pos: 6..7,
                    token: Token::Symbol(Symbol::Multiply)
                },
                PosToken {
                    file_id: 0,
                    pos: 8..9,
                    token: Token::Symbol(Symbol::Divide),
                },
                PosToken {
                    file_id: 0,
                    pos: 10..11,
                    token: Token::Symbol(Symbol::Remains),
                },
                PosToken {
                    file_id: 0,
                    pos: 12..14,
                    token: Token::Symbol(Symbol::Exponent),
                },
                PosToken {
                    file_id: 0,
                    pos: 15..17,
                    token: Token::Symbol(Symbol::Equals),
                },
                PosToken {
                    file_id: 0,
                    pos: 18..20,
                    token: Token::Symbol(Symbol::NotEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 21..22,
                    token: Token::Symbol(Symbol::LessThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 23..24,
                    token: Token::Symbol(Symbol::GreaterThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 25..27,
                    token: Token::Symbol(Symbol::LessThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 28..30,
                    token: Token::Symbol(Symbol::GreaterThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 31..32,
                    token: Token::Symbol(Symbol::Not),
                },
                PosToken {
                    file_id: 0,
                    pos: 33..34,
                    token: Token::Symbol(Symbol::And),
                },
                PosToken {
                    file_id: 0,
                    pos: 35..36,
                    token: Token::Symbol(Symbol::Or),
                },
                PosToken {
                    file_id: 0,
                    pos: 37..38,
                    token: Token::Symbol(Symbol::Xor),
                },
                PosToken {
                    file_id: 0,
                    pos: 39..41,
                    token: Token::Symbol(Symbol::LeftShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 42..44,
                    token: Token::Symbol(Symbol::RightShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 45..46,
                    token: Token::Symbol(Symbol::LeftParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 47..48,
                    token: Token::Symbol(Symbol::RightParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 49..50,
                    token: Token::Symbol(Symbol::LeftBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 51..52,
                    token: Token::Symbol(Symbol::RightBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 53..54,
                    token: Token::Symbol(Symbol::LeftBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 55..56,
                    token: Token::Symbol(Symbol::RightBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 57..58,
                    token: Token::Symbol(Symbol::Comma),
                },
                PosToken {
                    file_id: 0,
                    pos: 59..60,
                    token: Token::Symbol(Symbol::Dot),
                },
                PosToken {
                    file_id: 0,
                    pos: 61..62,
                    token: Token::Symbol(Symbol::Colon),
                },
                PosToken {
                    file_id: 0,
                    pos: 63..64,
                    token: Token::Symbol(Symbol::UnderLine),
                },
                PosToken {
                    file_id: 0,
                    pos: 65..66,
                    token: Token::Symbol(Symbol::At),
                },
            ]])
        );
    }
}
