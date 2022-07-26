//! Types for runtimes
use crate::ast::{Key, KeyKind};
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

/// Snapshots for the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<L> {
    pub root: Table<L>,
    pub errors: Vec<Error<L>>,
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
            Self::Array(_) => Kind::Array,
            Self::Table(_) => Kind::Table,
        }
    }
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

    /// Inserts an element
    pub fn insert(&mut self, key: Key<L>, value: Value<L>) -> Result<(), Error<L>> where L: Clone {
        let (table, used) = match key.kind {
            KeyKind::Normal => (&mut self.global, true),
            KeyKind::Local => (&mut self.global, false),
        };

        if table.contains_key(&key.name) && !table[&key.name].default {
            Err(Error::DuplicateKey { existing: table[&key.name].defined.clone(), found: key.span })
        } else {
            table.insert(key.name, Element {
                value,
                defined: key.span,
                default: false,
                used,
            });
            Ok(())
        }
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
