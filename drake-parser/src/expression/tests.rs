use drake_types::ast::{Expression, ExpressionKind, Literal};
use drake_types::token::{Literal as TokenLit, Token};
use somen::prelude::*;

use crate::test_utils::test_parser;

#[test]
fn expression() {
    test_parser(
        super::expression().complete(),
        &[
            (
                &[Token::Literal(TokenLit::Character('a'))],
                Some(Expression {
                    kind: ExpressionKind::Literal(Literal::Character('a')),
                    span: 0..1,
                }),
            ),
            (&[Token::Whitespaces], None),
        ],
    );
}
