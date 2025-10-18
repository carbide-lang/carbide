use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CarbideParserError {}
