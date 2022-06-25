//! Token types
use alloc::string::{String, ToString};
use core::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Newline => write!(f, "newline"),
            Token::Whitespaces => write!(f, "whitespaces"),
            Token::Comment(com) => write!(f, "// {com}"),
            Token::Symbol(sym) => sym.fmt(f),
            Token::Identifier(id) => id.fmt(f),
            Token::Literal(lit) => lit.fmt(f),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::BackSlash => write!(f, "\\"),
            Self::Underscore => write!(f, "_"),
            Self::At => write!(f, "@"),
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            IdentifierKind::Bare => self.name.fmt(f),
            IdentifierKind::Raw => write!(
                f,
                "${{{}}}",
                self.name.escape_debug().to_string().replace('{', "\\{")
            ),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i, Radix::Binary) => write!(f, "0b{i:b}"),
            Self::Integer(i, Radix::Octal) => write!(f, "0o{i:o}"),
            Self::Integer(i, Radix::Hexadecimal) => write!(f, "0o{i:x}"),
            Self::Integer(i, Radix::Decimal) => i.fmt(f),
            Self::Float(fl) => fl.fmt(f),
            Self::Character(c) => write!(f, "{c:?}"),
            Self::String(s, StringKind::Normal) => write!(f, "{s:?}"),
            Self::String(s, StringKind::Raw(n)) => {
                write!(f, "{0}{s:?}{0}", "\"".repeat(*n as usize - 1))
            }
        }
    }
}
