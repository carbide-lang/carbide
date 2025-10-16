use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideParserError {
    #[error("`{0}` is not an ASCII char!")]
    NonASCIIChar(char),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Invalid integer literal `{0}`: {1:#?}")]
    InvalidIntegerLiteral(String, ParseIntError),
    #[error("Invalid hex literal `{0}`: {1:#?}")]
    InvalidHexLiteral(String, ParseIntError),
    #[error("Invalid binary literal `{0}`: {1:#?}")]
    InvalidBinaryLiteral(String, ParseIntError),
}
