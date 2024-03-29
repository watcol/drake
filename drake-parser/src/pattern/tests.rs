use alloc::string::String;
use drake_types::ast::{Key, KeyKind, Pattern, PatternKind};
use drake_types::token::{Identifier, IdentifierKind, Symbol, Token};
use somen::prelude::*;

use crate::test_utils::test_parser;

#[test]
fn pattern() {
    test_parser(
        super::pattern().complete(),
        &[
            (
                &[
                    Token::Symbol(Symbol::At),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                ],
                Some(Pattern {
                    kind: PatternKind::Builtin(Key {
                        kind: KeyKind::Normal,
                        name: String::from("abc"),
                        span: 1..2,
                    }),
                    span: 0..2,
                }),
            ),
            (
                &[Token::Identifier(Identifier {
                    kind: IdentifierKind::Bare,
                    name: String::from("abc"),
                })],
                Some(Pattern {
                    kind: PatternKind::Key(Key {
                        kind: KeyKind::Normal,
                        name: String::from("abc"),
                        span: 0..1,
                    }),
                    span: 0..1,
                }),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}
