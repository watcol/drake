use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value<L> {
    Character(char),
    String(String),
    Integer(u64),
    Float(f64),
    Array(Vec<Value<L>>),
    Table(Table<L>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<L> {
    pub value: Value<L>,
    pub defined: Range<L>,
    pub used: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table<L> {
    pub global: HashMap<String, Variable<L>>,
    pub local: HashMap<String, Variable<L>>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<L> {
    DuplicateKey {
        existing: Range<L>,
        found: Range<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Range<L>,
    },
}
