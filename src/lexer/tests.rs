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
fn bools() {
    let code = "@true @false";
    assert_eq!(
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..5,
                token: Token::Bool(true),
            },
            PosToken {
                pos: 6..12,
                token: Token::Bool(false),
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
    let code = "0xd34db33f 0o644 0b10011110 42 1_2__3";
    assert_eq!(
        lexer(code),
        Ok(vec![
            PosToken {
                pos: 0..10,
                token: Token::Int(0xd34db33f),
            },
            PosToken {
                pos: 11..16,
                token: Token::Int(0o644),
            },
            PosToken {
                pos: 17..27,
                token: Token::Int(0b10011110),
            },
            PosToken {
                pos: 28..30,
                token: Token::Int(42),
            },
            PosToken {
                pos: 31..37,
                #[allow(clippy::inconsistent_digit_grouping)]
                token: Token::Int(1_2__3),
            },
        ])
    );
}

#[test]
fn floats() {
    let code1 = "1_2_3.0_2_3 23e-0_2 1.1e+2 2e2 @inf";
    assert_eq!(
        lexer(code1),
        Ok(vec![
            PosToken {
                pos: 0..11,
                token: Token::Float(123.023f64),
            },
            PosToken {
                pos: 12..19,
                token: Token::Float(23e-2f64),
            },
            PosToken {
                pos: 20..26,
                token: Token::Float(1.1e2f64),
            },
            PosToken {
                pos: 27..30,
                token: Token::Float(2e2f64),
            },
            PosToken {
                pos: 31..35,
                token: Token::Float(f64::INFINITY),
            },
        ])
    );
    let code2 = "@nan";
    assert!(matches!(
        lexer(code2).unwrap()[..],
        [PosToken {
            pos: std::ops::Range { start: 0, end: 4 },
            token: Token::Float(f),
        }] if f.is_nan()
    ));
}
