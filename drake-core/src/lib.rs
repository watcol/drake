#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use codespan_reporting::files::SimpleFiles;
use drake_types::ast::Statement;
use drake_types::token::Token;

/// A struct contains all runtime informations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Runtime {
    modules: Vec<Module>,
    files: SimpleFiles<String, Source>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            files: SimpleFiles::new(),
        }
    }

    pub fn add(&mut self, name: String, source: String) -> usize {
        if let Some((id, _)) = self
            .modules
            .iter()
            .enumerate()
            .find(|(_, m)| m.name == name)
        {
            return id;
        }

        let source = Source::from(source);
        let mod_id = self.files.add(name.clone(), source.clone());

        let module = Module::new(name, source);
        self.modules.push(module);
        mod_id
    }
}

/// A struct contains partial (or full) information while processing a module
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    name: String,
    source: Source,
    tokens: Option<Vec<Token>>,
    ast: Option<Vec<Statement<usize>>>,
}

impl Module {
    pub fn new(name: String, source: Source) -> Self {
        Self {
            name,
            source,
            tokens: None,
            ast: None,
        }
    }
}

/// A sharable form of source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source(Arc<String>);

impl From<String> for Source {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for Source {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
