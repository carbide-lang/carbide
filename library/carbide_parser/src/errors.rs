use ariadne::{Color, Label, Report, ReportKind};
use carbide_lexer::tokens::{SourceLocation, Token};

use carbide_errors::{codes::E1000, error::CarbideError};
use carbide_lexer::errors::ErrorSpan;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CarbideParserError {
    #[error("Unexpected end of input at [{0}]")]
    UnexpectedEOF(SourceLocation),

    #[error("Expected {expected}, but found {found}")]
    UnexpectedToken {
        expected: String,
        found: Token<'static>,
    },

    #[error("Expected identifier, but found {0}")]
    ExpectedIdentifier(Token<'static>),

    #[error("Expected expression at [{0}]")]
    ExpectedExpression(SourceLocation),

    #[error("Invalid assignment target at [{0}]")]
    InvalidAssignmentTarget(SourceLocation),

    #[error("Too many parameters in function declaration at [{0}]")]
    TooManyParameters(SourceLocation),

    #[error("Too many arguments in function call at [{0}]")]
    TooManyArguments(SourceLocation),

    #[error("Break statement outside of loop at [{0}]")]
    BreakOutsideLoop(SourceLocation),

    #[error("Continue statement outside of loop at [{0}]")]
    ContinueOutsideLoop(SourceLocation),

    #[error("Return statement outside of function at [{0}]")]
    ReturnOutsideFunction(SourceLocation),
}

impl CarbideError for CarbideParserError {
    type Span = ErrorSpan;

    fn code(&self) -> carbide_errors::codes::ErrCode {
        match self {
            _ => E1000,
        }
    }

    fn help(&self) -> Option<&'static str> {
        match self {
            _ => None,
        }
    }

    fn message(&self) -> String {
        format!("{self}")
    }

    fn report(&'_ self, file: &str, src: &str) -> Result<Report<'_, Self::Span>, Self>
    where
        Self: Sized,
    {
        match self {
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
}
