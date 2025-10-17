use std::fmt;
use std::ops::Range;

use crate::keywords::Keywords;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens<'a> {
    /// Integer literal, like `100`
    IntLiteral(i64),
    /// Float literal, like `0.5`
    FloatLiteral(f64),
    /// A hexadecimal literal, like `0xFF`
    HexLiteral(i64),
    /// A binary literal like `0b1010`
    BinaryLiteral(i64),
    /// An indentifier, like `my_ident`
    Identifier(&'a str),
    /// Whitespace
    Whitespace,
    /// A keyword, like `let` or `fn`
    Keyword(Keywords)
}

pub type Span = Range<u64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: Tokens<'a>,
    pub span: Span,
    pub src: &'a str,
}

impl<'a> Token<'a> {
    #[must_use]
    pub fn new(token_type: Tokens<'a>, span: Span, src: &'a str) -> Self {
        Self {
            token_type,
            span,
            src,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Tokens::{:?}@({}..{}) `{}`>",
            self.token_type, self.span.start, self.span.end, self.src
        )
    }
}
