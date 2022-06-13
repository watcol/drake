//! Token types
use alloc::string::String;

/// Values of tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A line break
    Newline,
    /// A sequence of whitespaces
    Whitespaces,
    /// A comment
    Comment(String),
    /// A symbol
    Symbol(Symbol),
    /// A key
    Key(Key),
    /// A literal
    Literal(Literal),
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
    /// A backslash (`\`, `U+005C`)
    BackSlash,
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
