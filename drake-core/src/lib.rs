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

impl Default for Runtime {
    #[inline]
    fn default() -> Self {
        Self {
            modules: Vec::new(),
            files: SimpleFiles::new(),
        }
    }
}

impl Runtime {
    /// Creates a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new module by the name and the source code.
    pub fn add(&mut self, name: String, source: String) -> usize {
        if let Some((id, _)) = self.get_module_by_name(&name) {
            return id;
        }

        let source = Source::from(source);
        let mod_id = self.files.add(name.clone(), source.clone());

        let module = Module::new(name, source);
        self.modules.push(module);
        mod_id
    }

    /// Gets a slice of modules indexed by identifiers.
    #[inline]
    pub fn get_modules(&self) -> &[Module] {
        self.modules.as_slice()
    }

    /// Gets a reference of a corresponding module by the given identifier.
    #[inline]
    pub fn get_module(&self, id: usize) -> Option<&Module> {
        self.modules.get(id)
    }

    /// Gets a reference of a corresponding module by the given name.
    #[inline]
    pub fn get_module_by_name<S: AsRef<str>>(&self, name: S) -> Option<(usize, &Module)> {
        self.modules
            .iter()
            .enumerate()
            .find(|(_, m)| m.name == name.as_ref())
    }

    /// Gets a mutable reference of a corresponding module by the module identifier.
    #[inline]
    pub fn get_mut_module(&mut self, id: usize) -> Option<&mut Module> {
        self.modules.get_mut(id)
    }

    /// Gets a mutable reference of a corresponding module by the given name.
    #[inline]
    pub fn get_mut_module_by_name<S: AsRef<str>>(
        &mut self,
        name: S,
    ) -> Option<(usize, &mut Module)> {
        self.modules
            .iter_mut()
            .enumerate()
            .find(|(_, m)| m.name == name.as_ref())
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
    /// Creates a new instance.
    #[inline]
    pub fn new(name: String, source: Source) -> Self {
        Self {
            name,
            source,
            tokens: None,
            ast: None,
        }
    }

    /// Tokenizes the module and returns a reference of tokens.
    pub async fn tokenize(&mut self) -> Result<&[Token], ParseError> {
        use futures_util::TryStreamExt;
        use somen::prelude::*;

        let mut input = stream::from_iter(self.source.as_ref().chars()).buffered_rewind();
        let mut lexer = drake_lexer::token().repeat(..);

        let tokens = lexer.parse_iterable(&mut input).try_collect().await?;
        self.tokens = Some(tokens);
        Ok(self.tokens.as_ref().unwrap())
    }

    /// Parses the module and returns a reference of the parsed AST.
    ///
    /// Note that this function also does tokenizing if it has not done yet.
    pub async fn parse(&mut self) -> Result<&[Statement<usize>], ParseError> {
        use futures_util::TryStreamExt;
        use somen::prelude::*;

        let ast = {
            let tokens = match self.tokens {
                Some(ref tokens) => tokens.as_slice(),
                None => self.tokenize().await?,
            };

            let mut input = stream::from_slice(tokens);
            let mut parser = drake_parser::statement::statement().repeat(..);

            parser.parse_iterable(&mut input).try_collect().await?
        };

        self.ast = Some(ast);
        Ok(self.ast.as_ref().unwrap())
    }

    /// Gets a reference for the name of the module.
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets a reference for the source code of the module.
    #[inline]
    pub fn get_source(&self) -> &str {
        self.source.as_ref()
    }

    /// Gets a reference for the tokenized result of the module.
    #[inline]
    pub fn get_tokens(&self) -> Option<&[Token]> {
        self.tokens.as_ref().map(|tokens| tokens.as_slice())
    }

    /// Gets a reference for the parsed AST of the module.
    #[inline]
    pub fn get_ast(&self) -> Option<&[Statement<usize>]> {
        self.ast.as_ref().map(|ast| ast.as_slice())
    }
}

/// An error occured while parsing or tokenizing.
pub enum ParseError {
    /// A tokenizing error
    Tokenize(somen::error::Error<usize>),
    /// A parsing error
    Parse(somen::error::Error<usize>),
    /// An unexpected error (probably an internal bug)
    Unexpected,
}

type OriginalTokenizeError = somen::error::ParseError<
    usize,
    somen::stream::rewind::BufferedError<core::convert::Infallible>,
>;

type OriginalParseError = somen::error::ParseError<usize, core::convert::Infallible>;

impl From<OriginalTokenizeError> for ParseError {
    #[inline]
    fn from(err: OriginalTokenizeError) -> Self {
        match err {
            somen::error::ParseError::Parser(e) => Self::Tokenize(e),
            _ => Self::Unexpected,
        }
    }
}

impl From<OriginalParseError> for ParseError {
    #[inline]
    fn from(err: OriginalParseError) -> Self {
        match err {
            somen::error::ParseError::Parser(e) => Self::Parse(e),
            _ => Self::Unexpected,
        }
    }
}

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
