//! Types for runtimes
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

/// Evaluated values
#[derive(Clone, Debug, PartialEq)]
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
}
