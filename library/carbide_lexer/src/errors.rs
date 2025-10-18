use std::num::{ParseFloatError, ParseIntError, TryFromIntError};

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideLexerError {
    #[error("`{0}` is not an ASCII char!")]
    NonASCIIChar(char),

    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Unexpected character `{0}`")]
    UnexpectedChar(char),

    #[error("Invalid float literal `{0}`: {1:#?}")]
    InvalidFloatLiteral(String, ParseFloatError),
    #[error("Invalid integer literal `{0}`: {1:#?}")]
    InvalidIntegerLiteral(String, ParseIntError),
    #[error("Invalid hex literal `{0}`: {1:#?}")]
    InvalidHexLiteral(String, ParseIntError),
    #[error("Invalid binary literal `{0}`: {1:#?}")]
    InvalidBinaryLiteral(String, ParseIntError),

    #[error("Failed to cast `{0}` as `{1}`: {2:#?}")]
    CastIntFailed(String, String, TryFromIntError),
    #[error("Failed to cast `{0}` as a keyword")]
    CastKeywordFailed(String),
    #[error("Failed to cast `{0}` as a binary operator")]
    CastBinaryOpFailed(String),
    #[error("Failed to cast `{0}` as a unary operator")]
    CastUnaryOpFailed(String),
}
