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
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 10..13,
                token: Token::Char('a'),
            },
            PosToken {
                pos: 13..31,
                token: Token::Symbol(Symbol::Newline),
            },
            PosToken {
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
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..1,
                token: Token::Symbol(Symbol::Dot)
            },
            PosToken {
                pos: 2..3,
                token: Token::Symbol(Symbol::UnderLine)
            },
            PosToken {
                pos: 4..5,
                token: Token::Symbol(Symbol::Assign),
            },
            PosToken {
                pos: 6..8,
                token: Token::Symbol(Symbol::Equals),
            },
            PosToken {
                pos: 9..10,
                token: Token::Symbol(Symbol::Not),
            },
            PosToken {
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
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..7,
                token: Token::Ident(String::from("f00_B4r")),
            },
            PosToken {
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
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..3,
                token: Token::Char('a'),
            },
            PosToken {
                pos: 4..8,
                token: Token::Char('\n'),
            },
            PosToken {
                pos: 9..13,
                token: Token::Char('\''),
            },
            PosToken {
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
        lexer(code1),
        Ok(vec![
            PosToken {
                pos: 0..2,
                token: Token::Str(String::from(""))
            },
            PosToken {
                pos: 3..23,
                token: Token::Str(String::from("\t\r\x00\n\"foo\""))
            }
        ])
    );
    let code2 = "\"\"\"\\not\\escaped\n\r\n\r\\\\\"\"\"";
    assert_eq!(
        lexer(code2),
        Ok(vec![PosToken {
            pos: 0..24,
            token: Token::Str(String::from("\\not\\escaped\n\n\n\\\\"))
        }])
    );
    let code3 = r#"""""(""")"""""#;
    assert_eq!(
        lexer(code3),
        Ok(vec![PosToken {
            pos: 0..13,
            token: Token::Str(String::from(r#"(""")"#)),
        }])
    );
}

#[test]
fn ints() {
    let code = "0xd34db33f +0o644 -0b10011110 42 -1_2__3";
    assert_eq!(
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..10,
                token: Token::Int(0xd34db33f),
            },
            PosToken {
                pos: 11..17,
                token: Token::Int(0o644),
            },
            PosToken {
                pos: 18..29,
                token: Token::Int(-0b10011110),
            },
            PosToken {
                pos: 30..32,
                token: Token::Int(42),
            },
            PosToken {
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
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..11,
                token: Token::Float(123.023f64),
            },
            PosToken {
                pos: 12..20,
                token: Token::Float(23e-2f64),
            },
            PosToken {
                pos: 21..28,
                token: Token::Float(-1.1e2f64),
            },
            PosToken {
                pos: 29..32,
                token: Token::Float(2e2f64),
            },
        ])
    );
}
