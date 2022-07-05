#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use codespan_reporting::files::SimpleFiles;
use drake_types::ast::Statement;
use drake_types::token::Token;

/// A context for runtime includes a loader and a source code database
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Context<L: ModuleLoader> {
    files: SimpleFiles<L::Name, Source>,
    loader: L,
}

/// A trait for loading modules
pub trait ModuleLoader {
    /// A type of module specifiers with a human-readable [`Display`] implementation
    type Name: core::fmt::Display;

    /// Loads a specified module into [`String`]
    fn load(name: &Self::Name) -> String;
}

/// A module which has partial (or full) information of processing
#[derive(Debug, Clone, PartialEq)]
pub struct Module<FileName> {
    name: FileName,
    source: Option<Source>,
    tokens: Option<Vec<Token>>,
    ast: Option<Vec<Statement<usize>>>,
}

/// A sharable form of source code indexed by [`Context`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source(Arc<String>);

impl AsRef<str> for Source {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
