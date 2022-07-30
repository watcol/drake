#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    Character,
    String,
    Integer,
    Float,
    Boolean,
    Null,
    Array,
    Table,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span<L> {
    pub file_id: usize,
    pub span: core::ops::Range<L>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error<L> {
    ParseError {
        expects: somen::error::Expects,
        span: Span<L>,
    },
    DuplicateKey {
        existing: Span<L>,
        found: Span<L>,
    },
    KindMismatch {
        expect: alloc::vec::Vec<Kind>,
        found: Kind,
        span: Span<L>,
    },
    BuiltinNotFound {
        span: Span<L>,
    },
    UnallowedDefaultValue {
        span: Span<L>,
    },
    NotSupported {
        feature: &'static str,
        span: Span<L>,
    },
    Unexpected,
}
