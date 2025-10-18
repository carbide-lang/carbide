use std::num::TryFromIntError;

use thiserror::Error;

use crate::tokens::SourceLocation;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideLexerError {
    #[error("Non ASCII char `{0}` at [{1}]")]
    NonASCIIChar(char, SourceLocation),

    #[error("Unexpected end of input at [{0}]")]
    UnexpectedEOF(SourceLocation),
    #[error("Unexpected character `{0}` at [{1}]")]
    UnexpectedChar(char, SourceLocation),

    #[error("Invalid float literal `{0}` at [{1}]")]
    InvalidFloatLiteral(String, SourceLocation),
    #[error("Invalid integer literal `{0}` at [{1}]")]
    InvalidIntegerLiteral(String, SourceLocation),
    #[error("Invalid hex literal `{0}` at [{1}]")]
    InvalidHexLiteral(String, SourceLocation),
    #[error("Invalid binary literal `{0}` at [{1}]")]
    InvalidBinaryLiteral(String, SourceLocation),

    #[error("Failed to cast `{0}` as `{1}`: {2:#?}")]
    CastIntFailed(String, String, TryFromIntError),
    #[error("Failed to cast `{0}` as a keyword")]
    CastKeywordFailed(String),
    #[error("Failed to cast `{0}` as a binary operator")]
    CastBinaryOpFailed(String),
    #[error("Failed to cast `{0}` as a unary operator")]
    CastUnaryOpFailed(String),

    #[error("Unclosed comment at [{0}]")]
    UnclosedComment(SourceLocation),
    #[error("Unclosed string at [{0}]")]
    UnclosedString(SourceLocation),
    #[error("Unmatched brace in interpolated string at [{0}]")]
    UnmatchedBrace(SourceLocation),
}
