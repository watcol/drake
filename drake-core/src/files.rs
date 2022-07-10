use alloc::string::String;
use alloc::sync::Arc;

/// A sharable form of source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source(Arc<String>);

impl From<String> for Source {
    #[inline]
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl core::fmt::Display for Source {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::ops::Deref for Source {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<str> for Source {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
