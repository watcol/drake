//! Types for parsers
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;

/// Statements
#[derive(Clone, Debug, PartialEq)]
pub struct Statement<L> {
    /// The kind of the statement
    pub kind: StatementKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

/// Kinds of statements
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum StatementKind<L> {
    /// A value binding like `pat = "expr"`
    ValueBinding(Pattern<L>, Expression<L>),
    /// A table header
    TableHeader(TableHeaderKind, Pattern<L>, Option<Expression<L>>),
}

/// Kinds of table headers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TableHeaderKind {
    /// A normal table header like `[table]`
    Normal,
    /// An array of tables like `[[array]]`
    Array,
}

/// Patterns
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<L> {
    /// The kind of the pattern
    pub kind: PatternKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

/// Kinds of patterns
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PatternKind<L> {
    /// A key pattern
    Key(Key<L>),
    /// A built-in pattern
    Builtin(Key<L>),
}

/// Keys
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key<L> {
    /// The kind of the key
    pub kind: KeyKind,
    /// The name of the key
    pub name: String,
    /// The range in the file
    pub span: Range<L>,
}

/// Kinds of keys
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyKind {
    /// A normal key like `key`
    Normal,
    /// A local key like `_key`
    Local,
}

/// Expressions
#[derive(Clone, Debug, PartialEq)]
pub struct Expression<L> {
    /// The kind of the expression
    pub kind: ExpressionKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

/// Kinds of expressions
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ExpressionKind<L> {
    /// A literal
    Literal(Literal),
    /// An array
    Array(Vec<Expression<L>>),
    /// An inline table
    InlineTable(Vec<(Key<L>, Expression<L>)>),
}

/// Values of literals
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Literal {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
}

impl<L> fmt::Display for Statement<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            StatementKind::ValueBinding(ref pat, ref exp) => write!(f, "{pat} = {exp}"),
            StatementKind::TableHeader(TableHeaderKind::Normal, ref pat, Some(ref exp)) => {
                write!(f, "[{pat} = {exp}]")
            }
            StatementKind::TableHeader(TableHeaderKind::Array, ref pat, Some(ref exp)) => {
                write!(f, "[[{pat} = {exp}]]")
            }
            StatementKind::TableHeader(TableHeaderKind::Normal, ref pat, None) => {
                write!(f, "[{pat}]")
            }
            StatementKind::TableHeader(TableHeaderKind::Array, ref pat, None) => {
                write!(f, "[[{pat}]]")
            }
        }
    }
}

impl<L> fmt::Display for Expression<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ExpressionKind::Literal(ref lit) => lit.fmt(f),
            ExpressionKind::Array(ref arr) => {
                write!(f, "[")?;
                for (i, elem) in arr.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    elem.fmt(f)?;
                }
                write!(f, "]")
            }
            ExpressionKind::InlineTable(ref table) => {
                write!(f, "{{")?;
                for (i, (pat, elem)) in table.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{pat} = {elem}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl<L> fmt::Display for Pattern<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            PatternKind::Key(ref key) => key.fmt(f),
            PatternKind::Builtin(ref key) => write!(f, "@{key}"),
        }
    }
}

impl<L> fmt::Display for Key<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            KeyKind::Normal => self.name.fmt(f),
            KeyKind::Local => write!(f, "_{}", self.name),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Character(c) => write!(f, "{c:?}"),
            Self::String(s) => write!(f, "{s:?}"),
            Self::Integer(i) => i.fmt(f),
            Self::Float(fl) => fl.fmt(f),
        }
    }
}
