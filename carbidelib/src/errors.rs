use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid token `{0}` at span `{1}..{2}`!")]
    InvalidToken(String, usize, usize),
    #[error("Invalid type `{0}`!")]
    InvalidType(String),
}

#[derive(Debug, Error)]
pub enum ASTError {
    #[error("Unexpected token `{1}`: `{0}{1}{2}`!")]
    UnexpectedToken(String, String, String),
    #[error("Unexpected EOF `{0}`!")]
    UnexpectedEOF(String),
    #[error("Unexpected EOI `{0}!`")]
    UnexpectedEOI(String),
    #[error("Syntax Error: `{0}` at `{1}`!")]
    SyntaxError(String, String),
    #[error("Parser Error `{0}`!")]
    ParserError(String),
    #[error("Assignment Error `{0}`!")]
    AssignmentError(String),
    #[error("Operator Error {0} is not a valid {1}!!")]
    OperatorError(String, String)
}
