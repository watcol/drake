//! Processing modules
mod parse;

use alloc::string::String;
use alloc::vec::Vec;
use codespan_reporting::files::Files;
use core::ops::Range;
use drake_ir::interpret;
use drake_types::ast::Statement;
use drake_types::error::Error;
use drake_types::ir::Ir;

use crate::files::Source;
pub use parse::Token;
use parse::{parse, tokenize};

/// A struct contains partial (or full) information while processing a module
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    file_id: usize,
    name: String,
    source: Source,
    tokens: Option<Vec<Token>>,
    ast: Option<Vec<Statement<usize>>>,
    ir: Option<Ir<usize>>,
    errors: Vec<Error<usize>>,
}

impl<'a> Files<'a> for Module {
    type FileId = ();
    type Name = &'a str;
    type Source = &'a str;

    #[inline]
    fn name(&'a self, _: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(&self.name)
    }

    #[inline]
    fn source(&'a self, _: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(self.source.as_ref())
    }

    #[inline]
    fn line_index(
        &'a self,
        _: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self.source.line_index(byte_index))
    }

    #[inline]
    fn line_range(
        &'a self,
        _: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, codespan_reporting::files::Error> {
        self.source.line_range(line_index).map_err(|max| {
            codespan_reporting::files::Error::LineTooLarge {
                given: line_index,
                max,
            }
        })
    }
}

impl Module {
    /// Creates a new instance.
    #[inline]
    pub fn new(file_id: usize, name: String, source: String) -> Self {
        Self {
            file_id,
            name,
            source: Source::new(source),
            tokens: None,
            ast: None,
            ir: None,
            errors: Vec::new(),
        }
    }

    /// Tokenizes the module and returns a reference of tokens.
    pub async fn tokenize(&mut self) -> &[Token] {
        if let Some(ref tokens) = self.tokens {
            return tokens.as_slice();
        }

        let tokens = match tokenize(self.source.as_ref(), self.file_id).await {
            Ok(tokens) => tokens,
            Err(err) => {
                self.errors.push(err);
                Vec::new()
            }
        };

        self.tokens = Some(tokens);
        self.tokens.as_ref().unwrap()
    }

    /// Parses the module and returns a reference of the parsed AST.
    ///
    /// Note that this function also does tokenizing if it has not done yet.
    pub async fn parse(&mut self) -> &[Statement<usize>] {
        if let Some(ref ast) = self.ast {
            return ast.as_slice();
        }

        let id = self.file_id;
        let ast = match parse(self.tokenize().await, id).await {
            Ok(ast) => ast,
            Err(err) => {
                self.errors.push(err);
                Vec::new()
            }
        };

        self.ast = Some(ast);
        self.ast.as_ref().unwrap()
    }

    /// Interprets the module and returns a reference of IR.
    pub async fn evaluate(&mut self) -> &Ir<usize> {
        let id = self.file_id;
        let (ir, mut errors) = interpret(self.parse().await, id);

        self.errors.append(&mut errors);
        self.ir = Some(ir);
        self.ir.as_ref().unwrap()
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
        self.tokens.as_deref()
    }

    /// Gets a reference for the parsed AST of the module.
    #[inline]
    pub fn get_ast(&self) -> Option<&[Statement<usize>]> {
        self.ast.as_deref()
    }

    /// Gets a reference for the interpreted IR.
    #[inline]
    pub fn get_ir(&self) -> Option<&Ir<usize>> {
        self.ir.as_ref()
    }
}
