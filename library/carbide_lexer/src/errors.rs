use std::num::TryFromIntError;

use ariadne::{Color, Label, Report, ReportKind};
use carbide_errors::{
    codes::{E0000, E0001, E0002, E0003, E0004, E0005, E0006, ErrCode},
    error::CarbideError,
};
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

/// A span type that implements [`ariadne::Span`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorSpan {
    source_id: String,
    start: usize,
    end: usize,
}

impl ErrorSpan {
    pub fn new(source_id: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            source_id: source_id.into(),
            start,
            end: end.max(start),
        }
    }
}

impl ariadne::Span for ErrorSpan {
    type SourceId = String;

    fn source(&self) -> &Self::SourceId {
        &self.source_id
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

impl CarbideError for CarbideLexerError {
    type Span = ErrorSpan;

    fn code(&self) -> ErrCode {
        match self {
            Self::NonASCIIChar(_, _) => E0001,
            Self::UnexpectedEOF(_) => E0002,
            Self::UnexpectedChar(_, _) => E0003,
            Self::UnclosedString(_) => E0004,
            Self::UnmatchedBrace(_) => E0005,
            Self::InvalidFloatLiteral(_, _)
            | Self::InvalidIntegerLiteral(_, _)
            | Self::InvalidHexLiteral(_, _)
            | Self::InvalidBinaryLiteral(_, _) => E0006,
            _ => E0000,
        }
    }

    fn message(&self) -> String {
        format!("{self}")
    }

    #[allow(clippy::too_many_lines)]
    fn report(&'_ self, file: &str, src: &str) -> Result<Report<'_, ErrorSpan>, Self> {
        match self {
            Self::NonASCIIChar(ch, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = ch.len_utf8();

                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message(format!("Remove '{ch}'"))
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Only ASCII characters are allowed in source code")
                    .finish())
            }

            Self::UnexpectedChar(ch, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = ch.len_utf8();

                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message(format!("Remove '{ch}'"))
                            .with_color(Color::BrightRed),
                    )
                    .with_help("This character is not valid in this context")
                    .finish())
            }

            Self::UnclosedString(loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let error_span = ErrorSpan::new(file, offset, offset + 1);

                let string_end = src[offset..]
                    .find('\n')
                    .map_or(src.len(), |pos| offset + pos);

                let suggestion_span = ErrorSpan::new(file, string_end, string_end + 1);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("String starts here")
                            .with_color(Color::BrightRed),
                    )
                    .with_label(
                        Label::new(suggestion_span)
                            .with_message("Add closing \" here")
                            .with_color(Color::Green),
                    )
                    .with_help("Strings must be closed with a quote")
                    .finish())
            }

            Self::InvalidHexLiteral(lit, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = lit.len();

                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Add hex digits after '0x'")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Hex literals must have at least one digit (0-9, a-f, A-F)")
                    .finish())
            }

            Self::InvalidBinaryLiteral(lit, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = lit.len();

                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Add binary digits after '0b'")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Binary literals must have at least one digit (0 or 1)")
                    .with_note("Examples: 0b1010, 0b11111111, 0b0")
                    .finish())
            }

            Self::UnmatchedBrace(loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let error_span = ErrorSpan::new(file, offset, offset + 1);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Add closing '}' for this '{'")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Each '{' in string interpolation needs a matching '}'")
                    .with_note("String interpolation syntax: \"Hello {name}\"")
                    .finish())
            }

            Self::UnclosedComment(loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let error_span = ErrorSpan::new(file, offset, offset + 2);

                let suggestion_span = ErrorSpan::new(file, src.len(), src.len());

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Block comment starts here")
                            .with_color(Color::BrightRed),
                    )
                    .with_label(
                        Label::new(suggestion_span)
                            .with_message("Add '*/' here")
                            .with_color(Color::Green),
                    )
                    .with_help("Block comments must be closed with '*/'")
                    .finish())
            }

            Self::UnexpectedEOF(loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let error_span = ErrorSpan::new(file, offset.saturating_sub(1), offset);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("File ends unexpectedly here")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Check for unclosed strings, comments, or brackets")
                    .finish())
            }

            Self::InvalidFloatLiteral(lit, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = lit.len();
                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Fix the float format")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Floats can only have one decimal point")
                    .with_note("Valid examples: 3.14, 0.5, 1.0")
                    .finish())
            }

            Self::InvalidIntegerLiteral(lit, loc) => {
                let offset = usize::try_from(loc.offset).map_err(|e| {
                    CarbideLexerError::CastIntFailed(loc.offset.to_string(), "usize".to_string(), e)
                })?;

                let len = lit.len();
                let error_span = ErrorSpan::new(file, offset, offset + len);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Fix the integer format")
                            .with_color(Color::BrightRed),
                    )
                    .with_help("Integer is too large or has invalid characters")
                    .with_note("Valid examples: 42, 0, 123")
                    .finish())
            }

            _ => {
                let error_span = ErrorSpan::new(file, 0, 1);

                Ok(Report::build(ReportKind::Error, error_span.clone())
                    .with_code(self.code().to_string())
                    .with_message(self.message())
                    .with_label(
                        Label::new(error_span)
                            .with_message("Error occurred here")
                            .with_color(Color::BrightRed),
                    )
                    .finish())
            }
        }
    }

    fn help(&self) -> Option<&'static str> {
        match self {
            Self::UnclosedString(_) => Some("Strings must be closed with a quote"),
            Self::UnexpectedChar(_, _) => Some("This character is not valid in this context"),
            Self::NonASCIIChar(_, _) => Some("Only ASCII characters are allowed"),
            Self::UnmatchedBrace(_) => Some("Each '{' needs a matching '}'"),
            Self::UnclosedComment(_) => Some("Block comments must be closed with '*/'"),
            Self::InvalidHexLiteral(_, _) => Some("Hex literals must have at least one digit"),
            Self::InvalidBinaryLiteral(_, _) => {
                Some("Binary literals must have at least one digit")
            }
            Self::InvalidFloatLiteral(_, _) => Some("Floats can only have one decimal point"),
            Self::InvalidIntegerLiteral(_, _) => Some("Integer is invalid"),
            _ => None,
        }
    }
}
