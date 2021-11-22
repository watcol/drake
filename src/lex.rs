use std::ops::Range;

peg::parser! { grammar lexer() for str {
    rule symbol() -> Symbol = c:$(['\n'|'\r'|'='|'+'|'-'|'*'|'/'|'%'|
                                 '^'|'<'|'>'|'&'|'|'|'('|')'|'{'|
                                 '}'|'['|']'|','|'.'|':'|'_'|'@'])
    {
        match c {
            "\n"|"\r" => Symbol::NewLine,
            "=" => Symbol::Equals,
            "+" => Symbol::Plus,
            "-" => Symbol::Minus,
            "*" => Symbol::Multiply,
            "/" => Symbol::Divide,
            "%" => Symbol::Remains,
            "^" => Symbol::Exponent,
            "<" => Symbol::LessThan,
            ">" => Symbol::GreaterThan,
            "&" => Symbol::And,
            "|" => Symbol::Or,
            "(" => Symbol::LeftParenthesis,
            ")" => Symbol::RightParenthesis,
            "{" => Symbol::LeftBrace,
            "}" => Symbol::RightBrace,
            "[" => Symbol::LeftBracket,
            "]" => Symbol::RightBracket,
            "," => Symbol::Comma,
            "." => Symbol::Dot,
            ":" => Symbol::Colon,
            "_" => Symbol::UnderLine,
            "@" => Symbol::At,
            _ => unreachable!(),
        }
    }

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
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Remains,
    Exponent,
    LessThan,
    GreaterThan,
    And,
    Or,
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

fn lex(code: &str, file_id: usize) -> Vec<PosToken> {
    lexer::tokens(code, file_id).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols() {
        let code = " \t \n\r = + - * / % ^ < > & | ( ) { } [ ] , . : _ @ ";
        assert_eq!(
            lex(code, 0),
            vec![
                PosToken {
                    file_id: 0,
                    pos: 3..4,
                    token: Token::Symbol(Symbol::NewLine)
                },
                PosToken {
                    file_id: 0,
                    pos: 4..5,
                    token: Token::Symbol(Symbol::NewLine)
                },
                PosToken {
                    file_id: 0,
                    pos: 6..7,
                    token: Token::Symbol(Symbol::Equals)
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
                    pos: 18..19,
                    token: Token::Symbol(Symbol::Exponent),
                },
                PosToken {
                    file_id: 0,
                    pos: 20..21,
                    token: Token::Symbol(Symbol::LessThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 22..23,
                    token: Token::Symbol(Symbol::GreaterThan),
                },
                PosToken {
                    file_id: 0,
                    pos: 24..25,
                    token: Token::Symbol(Symbol::And),
                },
                PosToken {
                    file_id: 0,
                    pos: 26..27,
                    token: Token::Symbol(Symbol::Or),
                },
                PosToken {
                    file_id: 0,
                    pos: 28..29,
                    token: Token::Symbol(Symbol::LeftParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 30..31,
                    token: Token::Symbol(Symbol::RightParenthesis),
                },
                PosToken {
                    file_id: 0,
                    pos: 32..33,
                    token: Token::Symbol(Symbol::LeftBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 34..35,
                    token: Token::Symbol(Symbol::RightBrace),
                },
                PosToken {
                    file_id: 0,
                    pos: 36..37,
                    token: Token::Symbol(Symbol::LeftBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 38..39,
                    token: Token::Symbol(Symbol::RightBracket),
                },
                PosToken {
                    file_id: 0,
                    pos: 40..41,
                    token: Token::Symbol(Symbol::Comma),
                },
                PosToken {
                    file_id: 0,
                    pos: 42..43,
                    token: Token::Symbol(Symbol::Dot),
                },
                PosToken {
                    file_id: 0,
                    pos: 44..45,
                    token: Token::Symbol(Symbol::Colon),
                },
                PosToken {
                    file_id: 0,
                    pos: 46..47,
                    token: Token::Symbol(Symbol::UnderLine),
                },
                PosToken {
                    file_id: 0,
                    pos: 48..49,
                    token: Token::Symbol(Symbol::At),
                },
            ]
        );
    }
}
