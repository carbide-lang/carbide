use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IOError {
    #[error("Missing file: {0} at {1}")]
    MissingFile(String, PathBuf),
}