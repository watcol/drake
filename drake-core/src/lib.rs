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

    pub async fn tokenize(&mut self) -> Result<&[Token], ParseError> {
        use futures_util::TryStreamExt;
        use somen::prelude::*;

        let mut input = stream::from_iter(self.source.as_ref().chars()).buffered_rewind();
        let mut lexer = drake_lexer::token().repeat(..);

        let tokens = lexer.parse_iterable(&mut input).try_collect().await?;
        self.tokens = Some(tokens);
        Ok(self.tokens.as_ref().unwrap())
    }

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
}

pub enum ParseError {
    Tokenize(somen::error::Error<usize>),
    Parse(somen::error::Error<usize>),
    Unexpected,
}

type OriginalTokenizeError = somen::error::ParseError<
    usize,
    somen::stream::rewind::BufferedError<core::convert::Infallible>,
>;

type OriginalParseError = somen::error::ParseError<usize, core::convert::Infallible>;

impl From<OriginalTokenizeError> for ParseError {
    fn from(err: OriginalTokenizeError) -> Self {
        match err {
            somen::error::ParseError::Parser(e) => Self::Tokenize(e),
            _ => Self::Unexpected,
        }
    }
}

impl From<OriginalParseError> for ParseError {
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
