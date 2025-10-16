use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideParserError {
    #[error("`{0}` is not an ASCII char!")]
    NonASCIIChar(char),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Failed to parse integer `{0}`: {1:#?}")]
    InvalidInt(String, ParseIntError),
}