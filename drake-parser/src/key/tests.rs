use alloc::string::String;
use drake_types::ast::{Key, KeyKind};
use drake_types::token::{Identifier, IdentifierKind, Symbol, Token};
use somen::prelude::*;

use crate::test_utils::test_parser;

#[test]
fn key() {
    test_parser(
        super::key().complete(),
        &[
            (
                &[Token::Identifier(Identifier {
                    kind: IdentifierKind::Bare,
                    name: String::from("abc"),
                })],
                Some(Key {
                    kind: KeyKind::Normal,
                    name: String::from("abc"),
                    span: 0..1,
                }),
            ),
            (
                &[
                    Token::Symbol(Symbol::Underscore),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                ],
                Some(Key {
                    kind: KeyKind::Local,
                    name: String::from("abc"),
                    span: 0..2,
                }),
            ),
            (
                &[
                    Token::Symbol(Symbol::Assign),
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                ],
                None,
            ),
            (
                &[
                    Token::Symbol(Symbol::Underscore),
                    Token::Whitespaces,
                    Token::Identifier(Identifier {
                        kind: IdentifierKind::Bare,
                        name: String::from("abc"),
                    }),
                ],
                None,
            ),
        ],
    );
}
