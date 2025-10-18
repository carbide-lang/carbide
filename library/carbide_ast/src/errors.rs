use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideASTError {
    #[error("Expected `{0}` got `{1}`")]
    ExpectedGot(String, String),
}
