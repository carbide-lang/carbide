use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid token `{0}` at span `{1}..{2}`!")]
    InvalidToken(String, usize, usize),
    #[error("Invalid type `{0}`")]
    InvalidType(String),
}

#[derive(Debug, Error)]
pub enum ASTError {
    #[error("Unexpected token `{1}`: `{0}{1}{2}`!")]
    UnexpectedToken(String, String, String),
}
