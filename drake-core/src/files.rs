use core::ops::Range;

use alloc::string::String;
use alloc::vec::Vec;

/// A source code with line information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    source: String,
    line_starts: Vec<usize>,
}

impl From<String> for Source {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<Source> for String {
    #[inline]
    fn from(src: Source) -> Self {
        src.into_inner()
    }
}

impl Source {
    pub fn new(source: String) -> Self {
        let line_starts = core::iter::once(0)
            .chain(source.match_indices('\n').map(|(n, _)| n + 1))
            .collect();

        Self {
            source,
            line_starts,
        }
    }

    #[inline]
    pub fn into_inner(self) -> String {
        self.source
    }

    pub(crate) fn line_index(&self, idx: usize) -> usize {
        self.line_starts
            .binary_search(&idx)
            .unwrap_or_else(|next| next - 1)
    }

    pub(crate) fn line_range(&self, line: usize) -> Result<Range<usize>, usize> {
        use core::cmp::Ordering;

        match self.line_starts.len().cmp(&(line + 1)) {
            Ordering::Less => Ok(self.line_starts[line]..self.line_starts[line + 1]),
            Ordering::Equal => Ok(self.line_starts[line]..self.source.len()),
            Ordering::Greater => Err(self.line_starts.len() - 1),
        }
    }
}

impl core::fmt::Display for Source {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.source.fmt(f)
    }
}

impl AsRef<str> for Source {
    #[inline]
    fn as_ref(&self) -> &str {
        self.source.as_ref()
    }
}
