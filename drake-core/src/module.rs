mod parse;

use alloc::string::String;
use alloc::vec::Vec;
use codespan_reporting::files::{Error, Files};
use core::ops::Range;
use drake_types::ast::Statement;
use drake_types::token::Token;

use crate::files::Source;
pub use parse::ParseError;
use parse::{parse, tokenize};

/// A struct contains partial (or full) information while processing a module
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    name: String,
    source: Source,
    tokens: Option<Vec<Token>>,
    ast: Option<Vec<Statement<usize>>>,
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
    fn line_range(&'a self, _: Self::FileId, line_index: usize) -> Result<Range<usize>, Error> {
        self.source
            .line_range(line_index)
            .map_err(|max| Error::LineTooLarge {
                given: line_index,
                max,
            })
    }
}

impl Module {
    /// Creates a new instance.
    #[inline]
    pub fn new(name: String, source: String) -> Self {
        Self {
            name,
            source: Source::new(source),
            tokens: None,
            ast: None,
        }
    }

    /// Tokenizes the module and returns a reference of tokens.
    pub async fn tokenize(&mut self) -> Result<&[Token], ParseError> {
        self.tokens = Some(tokenize(self.source.as_ref()).await?);
        Ok(self.tokens.as_ref().unwrap())
    }

    /// Parses the module and returns a reference of the parsed AST.
    ///
    /// Note that this function also does tokenizing if it has not done yet.
    pub async fn parse(&mut self) -> Result<&[Statement<usize>], ParseError> {
        self.ast = Some(
            parse(match self.tokens {
                Some(ref tokens) => tokens.as_slice(),
                None => self.tokenize().await?,
            })
            .await?,
        );

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
        self.tokens.as_deref()
    }

    /// Gets a reference for the parsed AST of the module.
    #[inline]
    pub fn get_ast(&self) -> Option<&[Statement<usize>]> {
        self.ast.as_deref()
    }
}
