use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid token `{0}` at span `{1}..{2}`!")]
    InvalidToken(String, usize, usize),
    #[error("Invalid type `{0}`")]
    InvalidType(String)
}
