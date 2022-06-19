//! Types for parsers
use alloc::string::String;
use alloc::vec::Vec;
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
pub enum StatementKind<L> {
    /// A value binding like `pat = "expr"`
    ValueBinding(Pattern<L>, Expression<L>),
    /// A table header
    TableHeader(TableHeaderKind, Pattern<L>, Option<Expression<L>>),
}

/// Kinds of table headers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub enum PatternKind<L> {
    /// A key pattern
    Key(Key<L>),
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
pub enum KeyKind {
    /// A normal key like `key`
    Normal,
    /// A local key like `_key`
    Local,
    /// A built-in key like `@key`
    Builtin,
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
pub enum Literal {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
}
