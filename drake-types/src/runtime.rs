//! Types for runtimes
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

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
    /// An array
    Array(Vec<Value<L>>),
    /// A table
    Table(Table<L>),
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

/// Evaluated tables
#[derive(Clone, Debug, PartialEq)]
pub struct Table<L> {
    pub global: HashMap<String, Element<L>>,
    pub local: HashMap<String, Element<L>>,
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

/// Snapshots for the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<L> {
    pub root: Table<L>,
    pub errors: Vec<Error<L>>,
}

/// Errors for runtimes
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error<L> {
    DuplicateKey {
        existing: Range<L>,
        found: Range<L>,
    },
    KindMismatch {
        expect: Vec<Kind>,
        found: Kind,
        span: Range<L>,
    },
    UnallowedDefaultValue {
        span: Range<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Range<L>,
    },
    Unexpected,
}

/// Name of value kinds for errors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    Character,
    String,
    Integer,
    Float,
    Array,
    Table,
}

impl Kind {
    /// Evaluates a kind from the value.
    pub fn from_value<L>(val: &Value<L>) -> Self {
        match val {
            Value::Character(_) => Self::Character,
            Value::String(_) => Self::String,
            Value::Integer(_) => Self::Integer,
            Value::Float(_) => Self::Float,
            Value::Array(_) => Self::Array,
            Value::Table(_) => Self::Table,
        }
    }
}
