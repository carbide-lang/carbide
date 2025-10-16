use std::num::ParseIntError;

use thiserror::Error;

use crate::tokens::Token;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideParserError {
    #[error("Invalid integer: `{0:#?}`")]
    InvalidInteger(ParseIntError),
    #[error("Unexpected char: `{0}`")]
    UnexpectedChar(char),
    #[error("Other: {0}")]
    Other(String),
}

impl Default for CarbideParserError {
    fn default() -> Self {
        Self::Other("".to_string())
    }
}

impl CarbideParserError {
    pub fn from_lexer(lex: &mut logos::Lexer<'_, Token>) -> Self {
        CarbideParserError::UnexpectedChar(unsafe { lex.slice().chars().next().unwrap_unchecked() })
    }
}

impl From<ParseIntError> for CarbideParserError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidInteger(err)
    }
}
