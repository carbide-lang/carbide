use ariadne::{Color, Label, Report, ReportKind, Span};
use carbide_errors::{
    codes::{E1000, E1001, E1002, E1010, E1011, E1020, E1021, E1030, E1040, E1041, E1042},
    error::CarbideError,
};
use carbide_lexer::errors::ErrorSpan;
use carbide_lexer::tokens::{SourceLocation, Token};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CarbideParserError {
    #[error("Unexpected end of file at [{0}]")]
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

    #[error("Cast `{0}` as `{1}` failed!")]
    CastFailed(String, String),
}

impl CarbideError for CarbideParserError {
    type Span = ErrorSpan;

    fn code(&self) -> carbide_errors::codes::ErrCode {
        match self {
            Self::UnexpectedEOF(_) => E1001,
            Self::UnexpectedToken { .. } => E1002,
            Self::ExpectedIdentifier(_) => E1010,
            Self::ExpectedExpression(_) => E1011,
            Self::TooManyParameters(_) => E1020,
            Self::TooManyArguments(_) => E1021,
            Self::InvalidAssignmentTarget(_) => E1030,
            Self::BreakOutsideLoop(_) => E1040,
            Self::ContinueOutsideLoop(_) => E1041,
            Self::ReturnOutsideFunction(_) => E1042,
            _ => E1000,
        }
    }

    fn help(&self) -> Option<&'static str> {
        match self {
            Self::UnexpectedEOF(_) => {
                Some("Try closing any unclosed parentheses, braces, or quotes.")
            }
            Self::UnexpectedToken { .. } => {
                Some("Check for missing operators, delimiters, or keywords.")
            }
            Self::ExpectedIdentifier(_) => {
                Some("Identifiers must start with a letter or underscore.")
            }
            Self::ExpectedExpression(_) => {
                Some("You might have forgotten to include a value or expression.")
            }
            Self::InvalidAssignmentTarget(_) => {
                Some("Only variables or fields can appear on the left side of an assignment.")
            }
            Self::TooManyParameters(_) => {
                Some("Reduce the number of parameters to fit within the allowed limit.")
            }
            Self::TooManyArguments(_) => {
                Some("Reduce the number of arguments to match the function signature.")
            }
            Self::BreakOutsideLoop(_) => Some("`break` can only appear inside a loop."),
            Self::ContinueOutsideLoop(_) => Some("`continue` can only appear inside a loop."),
            Self::ReturnOutsideFunction(_) => Some("`return` can only appear inside a function."),
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
        let make_span = |loc: &SourceLocation| -> Result<ErrorSpan, Self> {
            let offset = usize::try_from(loc.offset).map_err(|_| {
                CarbideParserError::CastFailed(loc.offset.to_string(), "usize".to_string())
            })?;
            Ok(ErrorSpan::new(file, offset.saturating_sub(1), offset))
        };

        let mut report = match self {
            Self::UnexpectedEOF(loc) => {
                let span = make_span(loc)?;
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message("Unexpected end of file")
                    .with_label(
                        Label::new(span.clone())
                            .with_message("File ended here, but more code was expected")
                            .with_color(Color::BrightRed),
                    )
                    .with_note("Try closing unclosed blocks, strings, or parentheses.")
            }

            Self::UnexpectedToken { expected, found } => {
                let range = found.span.clone();
                let span = ErrorSpan::new(file, range.start as usize, range.end as usize);
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message(format!("Unexpected token `{}`", found.src))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!("Expected `{expected}` here"))
                            .with_color(Color::BrightRed),
                    )
                    .with_note(format!("Found token of type `{:?}`", found.token_type))
            }

            Self::ExpectedIdentifier(found) => {
                let span = ErrorSpan::new(file, found.span.start as usize, found.span.end as usize);
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message("Expected identifier")
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!("Found `{}` instead", found.src))
                            .with_color(Color::BrightRed),
                    )
                    .with_note("Identifiers must start with a letter or underscore.")
            }

            Self::ExpectedExpression(loc) => {
                let span = make_span(loc)?;
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message("Expected expression")
                    .with_label(
                        Label::new(span.clone())
                            .with_message("An expression is missing here")
                            .with_color(Color::BrightRed),
                    )
                    .with_note("Try adding a value, literal, or variable.")
            }

            Self::InvalidAssignmentTarget(loc) => {
                let span = make_span(loc)?;
                let snippet = src
                    .get(span.start()..span.end().min(src.len()))
                    .unwrap_or("");
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message("Invalid assignment target")
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!("`{snippet}` cannot be assigned to"))
                            .with_color(Color::BrightRed),
                    )
                    .with_note("Only variables or properties can be assigned.")
            }

            Self::TooManyParameters(loc) | Self::TooManyArguments(loc) => {
                let span = make_span(loc)?;
                let msg = match self {
                    Self::TooManyParameters(_) => "Too many parameters in function declaration",
                    _ => "Too many arguments in function call",
                };
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message(msg)
                    .with_label(
                        Label::new(span.clone())
                            .with_message("Exceeded allowed limit")
                            .with_color(Color::BrightRed),
                    )
            }

            Self::BreakOutsideLoop(loc)
            | Self::ContinueOutsideLoop(loc)
            | Self::ReturnOutsideFunction(loc) => {
                let span = make_span(loc)?;
                let msg = match self {
                    Self::BreakOutsideLoop(_) => "`break` outside of loop",
                    Self::ContinueOutsideLoop(_) => "`continue` outside of loop",
                    Self::ReturnOutsideFunction(_) => "`return` outside of function",
                    _ => unreachable!(),
                };
                Report::build(ReportKind::Error, span.clone())
                    .with_code(self.code().to_string())
                    .with_message(msg)
                    .with_label(
                        Label::new(span.clone())
                            .with_message("Not allowed in this context")
                            .with_color(Color::BrightRed),
                    )
            }

            other => {
                let span = ErrorSpan::new(file, 0, 1);
                Report::build(ReportKind::Error, span.clone())
                    .with_code(other.code().to_string())
                    .with_message(other.message())
                    .with_label(
                        Label::new(span.clone())
                            .with_message("Error occurred here")
                            .with_color(Color::BrightRed),
                    )
            }
        };

        if let Some(help) = self.help() {
            report = report.with_help(help);
        }

        Ok(report.finish())
    }
}
