//! Types for runtimes
use crate::error::Kind;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

/// Snapshots for the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<L> {
    pub root: Table<L>,
    pub builtin: Builtin,
}

/// Evaluated values
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Value<L> {
    /// A literal character
    Character(char),
    /// A literal string
    String(String),
    /// A literal integer
    Integer(u64),
    /// A literal float
    Float(f64),
    /// A boolean
    Boolean(bool),
    /// A null
    Null,
    /// An array
    Array(Vec<Value<L>>),
    /// A table
    Table(Table<L>),
}

impl<L> Value<L> {
    /// Returns a kind of the value
    pub fn kind(&self) -> Kind {
        match self {
            Self::Character(_) => Kind::Character,
            Self::String(_) => Kind::String,
            Self::Integer(_) => Kind::Integer,
            Self::Float(_) => Kind::Float,
            Self::Boolean(_) => Kind::Boolean,
            Self::Null => Kind::Null,
            Self::Array(_) => Kind::Array,
            Self::Table(_) => Kind::Table,
        }
    }
}

/// Evaluated tables
#[derive(Clone, Debug, PartialEq)]
pub struct Table<L> {
    pub global: HashMap<String, Element<L>>,
    pub local: HashMap<String, Element<L>>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub struct Builtin {
    pub output: Option<String>,
    pub filetype: Option<String>,
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
    /// Creates a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks the table as a default table.
    pub fn make_default(&mut self) {
        self.global
            .values_mut()
            .chain(self.local.values_mut())
            .for_each(|elem| elem.default = true);
    }
}

/// Table's elements
#[derive(Clone, Debug, PartialEq)]
pub struct Element<L> {
    /// A value of the element
    pub value: Value<L>,
    /// A position where the element is defined
    pub defined: Range<L>,
    /// A flag marks as the element is overridable.
    pub default: bool,
    /// A flag checks if the element is used, or not
    pub used: bool,
}
