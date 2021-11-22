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
        / "\n" { Symbol::NewLine }
        / "\r" { Symbol::NewLine }
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

    rule _ = [' '|'\t']*

    rule token(file_id: usize) -> PosToken
        = s:position!()
          t:(sym:symbol() { Token::Symbol(sym) })
          e:position!() { PosToken{ file_id, pos: s..e, token: t } }

    pub rule tokens(file_id: usize) -> Vec<PosToken>
        = _ ts:(token(file_id) ** _) _ { ts }
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
    NewLine,
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
             \t   \r = + - * / % ** == != < > <= >=
             ! & | ^ << >> ( ) { } [ ] , . : _ @ "};
        assert_eq!(
            lex(code, 0),
            Ok(vec![
                PosToken {
                    file_id: 0,
                    pos: 4..5,
                    token: Token::Symbol(Symbol::NewLine)
                },
                PosToken {
                    file_id: 0,
                    pos: 6..7,
                    token: Token::Symbol(Symbol::Assign)
                },
                PosToken {
                    file_id: 0,
                    pos: 8..9,
                    token: Token::Symbol(Symbol::Plus)
                },
                PosToken {
                    file_id: 0,
                    pos: 10..11,
                    token: Token::Symbol(Symbol::Minus)
                },
                PosToken {
                    file_id: 0,
                    pos: 12..13,
                    token: Token::Symbol(Symbol::Multiply)
                },
                PosToken {
                    file_id: 0,
                    pos: 14..15,
                    token: Token::Symbol(Symbol::Divide),
                },
                PosToken {
                    file_id: 0,
                    pos: 16..17,
                    token: Token::Symbol(Symbol::Remains),
                },
                PosToken {
                    file_id: 0,
                    pos: 18..20,
                    token: Token::Symbol(Symbol::Exponent),
                },
                PosToken {
                    file_id: 0,
                    pos: 21..23,
                    token: Token::Symbol(Symbol::Equals),
                },
                PosToken {
                    file_id: 0,
                    pos: 24..26,
                    token: Token::Symbol(Symbol::NotEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 27..28,
                    token: Token::Symbol(Symbol::LessThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 29..30,
                    token: Token::Symbol(Symbol::GreaterThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 31..33,
                    token: Token::Symbol(Symbol::LessThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 34..36,
                    token: Token::Symbol(Symbol::GreaterThanEquals),
                },
                PosToken {
                    file_id: 0,
                    pos: 36..37,
                    token: Token::Symbol(Symbol::NewLine),
                },
                PosToken {
                    file_id: 0,
                    pos: 37..38,
                    token: Token::Symbol(Symbol::Not),
                },
                PosToken {
                    file_id: 0,
                    pos: 39..40,
                    token: Token::Symbol(Symbol::And),
                },
                PosToken {
                    file_id: 0,
                    pos: 41..42,
                    token: Token::Symbol(Symbol::Or),
                },
                PosToken {
                    file_id: 0,
                    pos: 43..44,
                    token: Token::Symbol(Symbol::Xor),
                },
                PosToken {
                    file_id: 0,
                    pos: 45..47,
                    token: Token::Symbol(Symbol::LeftShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 48..50,
                    token: Token::Symbol(Symbol::RightShift),
                },
                PosToken {
                    file_id: 0,
                    pos: 51..52,
                    token: Token::Symbol(Symbol::LeftParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 53..54,
                    token: Token::Symbol(Symbol::RightParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 55..56,
                    token: Token::Symbol(Symbol::LeftBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 57..58,
                    token: Token::Symbol(Symbol::RightBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 59..60,
                    token: Token::Symbol(Symbol::LeftBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 61..62,
                    token: Token::Symbol(Symbol::RightBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 63..64,
                    token: Token::Symbol(Symbol::Comma),
                },
                PosToken {
                    file_id: 0,
                    pos: 65..66,
                    token: Token::Symbol(Symbol::Dot),
                },
                PosToken {
                    file_id: 0,
                    pos: 67..68,
                    token: Token::Symbol(Symbol::Colon),
                },
                PosToken {
                    file_id: 0,
                    pos: 69..70,
                    token: Token::Symbol(Symbol::UnderLine),
                },
                PosToken {
                    file_id: 0,
                    pos: 71..72,
                    token: Token::Symbol(Symbol::At),
                },
            ])
        );
    }
}
