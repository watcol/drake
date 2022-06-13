//! Token types
use alloc::string::String;
use core::ops::Range;

/// A token value and a position
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenValue,
    pub pos: Range<usize>,
}

impl PartialEq for Token {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialEq<TokenValue> for Token {
    #[inline]
    fn eq(&self, other: &TokenValue) -> bool {
        self.kind.eq(other)
    }
}

/// Values of tokens
#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    /// A line break
    Newline,
    /// A comment
    Comment(String),
    /// A symbol
    Symbol(Symbol),
    /// A key
    Key(Key),
    /// A literal
    Literal(Literal),
}

impl PartialEq<Token> for TokenValue {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        self.eq(&other.kind)
    }
}

/// Kinds of symbols
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    /// An assign sign (`=`, `U+003D`)
    Assign,
    /// A comma (`,`, `U+002C`)
    Comma,
    /// A dot (`.`, `U+002E`)
    Dot,
    /// An opening side of brackets (`[`, `U+005B`)
    OpenBracket,
    /// A closing side of brackets (`]`, `U+005D`)
    CloseBracket,
    /// A opening side of braces (`{`, `U+007B`)
    OpenBrace,
    /// A closing side of braces (`}`, `U+007D`)
    CloseBrace,
}

/// Kinds of keys
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    /// A normal (bare or raw) key
    Normal(String),
    /// A local key
    Local(String),
    /// A built-in key
    Builtin(String),
}

/// Literal values
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// An integer
    Integer(u64),
    /// An floating point decimal
    Float(f64),
    /// A character
    Character(char),
    /// A string
    String(String),
}
