mod parse;

use alloc::string::String;
use alloc::vec::Vec;
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
