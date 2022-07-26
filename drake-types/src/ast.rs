//! Types for parsers
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;

/// Statements
#[derive(Clone, Debug)]
pub struct Statement<L> {
    /// The kind of the statement
    pub kind: StatementKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

impl<L> PartialEq for Statement<L> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

/// Kinds of statements
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum StatementKind<L> {
    /// A value binding like `pat = "expr"`
    ValueBinding(Pattern<L>, Expression<L>),
    /// A table header
    TableHeader(TableHeaderKind, Pattern<L>, Option<Expression<L>>),
}

impl<L> PartialEq for StatementKind<L> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ValueBinding(pat1, expr1), Self::ValueBinding(pat2, expr2)) => {
                pat1 == pat2 && expr1 == expr2
            }
            (Self::TableHeader(kind1, pat1, def1), Self::TableHeader(kind2, pat2, def2)) => {
                kind1 == kind2 && pat1 == pat2 && def1 == def2
            }
            _ => false,
        }
    }
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
#[derive(Clone, Debug)]
pub struct Pattern<L> {
    /// The kind of the pattern
    pub kind: PatternKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

impl<L> PartialEq for Pattern<L> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<L> Eq for Pattern<L> {}

/// Kinds of patterns
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PatternKind<L> {
    /// A key pattern
    Key(Key<L>),
    /// A built-in pattern
    Builtin(Key<L>),
}

impl<L> PartialEq for PatternKind<L> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Key(key1), Self::Key(key2)) => key1 == key2,
            (Self::Builtin(key1), Self::Builtin(key2)) => key1 == key2,
            _ => false,
        }
    }
}

impl<L> Eq for PatternKind<L> {}

/// Keys
#[derive(Clone, Debug)]
pub struct Key<L> {
    /// The kind of the key
    pub kind: KeyKind,
    /// The name of the key
    pub name: String,
    /// The range in the file
    pub span: Range<L>,
}

impl<L> PartialEq for Key<L> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.name == other.name
    }
}

impl<L> Eq for Key<L> {}

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
#[derive(Clone, Debug)]
pub struct Expression<L> {
    /// The kind of the expression
    pub kind: ExpressionKind<L>,
    /// The range in the file
    pub span: Range<L>,
}

impl<L> PartialEq for Expression<L> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

/// Kinds of expressions
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ExpressionKind<L> {
    /// A literal
    Literal(Literal),
    /// An array
    Array(Vec<Expression<L>>),
    /// An inline table
    InlineTable(Vec<(Key<L>, Expression<L>)>),
}

impl<L> PartialEq for ExpressionKind<L> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(lit1), Self::Literal(lit2)) => lit1 == lit2,
            (Self::Array(arr1), Self::Array(arr2)) => arr1 == arr2,
            (Self::InlineTable(table1), Self::InlineTable(table2)) => table1 == table2,
            _ => false,
        }
    }
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
