use crate::ast::Literal;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Ir<L> {
    pub root: Table<Element<L>>,
    pub builtin: Builtin<L>,
}

impl<L> Default for Ir<L> {
    #[inline]
    fn default() -> Self {
        Self {
            root: Table::new(),
            builtin: Builtin::new(),
        }
    }
}

impl<L> Ir<L> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Builtin<L> {
    pub output: Option<Element<L>>,
    pub filetype: Option<Element<L>>,
}

impl<L> Default for Builtin<L> {
    #[inline]
    fn default() -> Self {
        Self {
            output: None,
            filetype: None,
        }
    }
}

impl<L> Builtin<L> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table<T> {
    pub global: HashMap<String, T>,
    pub local: HashMap<String, T>,
}

impl<L> Default for Table<L> {
    #[inline]
    fn default() -> Self {
        Self {
            global: HashMap::new(),
            local: HashMap::new(),
        }
    }
}

impl<L> Table<L> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element<L> {
    pub kind: ElementKind<L>,
    pub defined: Range<L>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind<L> {
    Table(Table<Element<L>>, Option<Expr<L>>),
    Array(Vec<Table<Element<L>>>, Option<Expr<L>>),
    Expr(Expr<L>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr<L> {
    pub kind: ExprKind<L>,
    pub span: Range<L>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind<L> {
    Literal(Literal),
    Array(Vec<Expr<L>>),
    Table(Table<Expr<L>>),
}
