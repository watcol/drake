use core::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<L> {
    pub kind: StatementKind<L>,
    pub span: Range<L>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind<L> {
    ValueBinding(Pattern<L>, Expression<L>),
    TableHeader(TableHeaderKind, Pattern<L>, Option<Expression<L>>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableHeaderKind {
    Normal,
    Array,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<L> {
    pub kind: PatternKind<L>,
    pub span: Range<L>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatternKind<L> {
    Key(Key<L>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key<L> {
    pub kind: KeyKind,
    pub name: String,
    pub span: Range<L>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyKind {
    Normal,
    Local,
    Builtin,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression<L> {
    pub kind: ExpressionKind<L>,
    pub span: Range<L>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind<L> {
    Literal(Literal),
    Array(Vec<Expression<L>>),
    InlineTable(Vec<(Key<L>, Expression<L>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
}
