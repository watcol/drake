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
    /// An identifier
    Identifier(Identifier),
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
    /// An underscore (`_`, `U+005F`)
    Underscore,
    /// An at mark (`@`, `U+0040`)
    At,
    /// An opening side of brackets (`[`, `U+005B`)
    OpenBracket,
    /// A closing side of brackets (`]`, `U+005D`)
    CloseBracket,
    /// A opening side of braces (`{`, `U+007B`)
    OpenBrace,
    /// A closing side of braces (`}`, `U+007D`)
    CloseBrace,
}

/// Identifiers
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    /// A kind of identifier
    pub kind: IdentifierKind,
    /// A name of identifier
    pub name: String,
}

/// Kinds of identifiers
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdentifierKind {
    /// A bare key
    Bare,
    /// A raw key
    Raw,
}

/// Literal values
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// An integer
    Integer(u64, Radix),
    /// An floating point decimal
    Float(f64),
    /// A character
    Character(char),
    /// A string
    String(String, StringKind),
}

/// Radixes of integers
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Radix {
    /// A binary integer starts with `0b`
    Binary,
    /// A octal integer starts with `0o`
    Octal,
    /// A hexadecimal integer starts with `0x`
    Hexadecimal,
    /// A decimal integer with no prefix
    Decimal,
}

/// Kinds of strings
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StringKind {
    /// A normal string surrounded by `""`
    Normal,
    /// A raw string surrounded by `"""""""` or more quotes
    Raw(u8),
}
