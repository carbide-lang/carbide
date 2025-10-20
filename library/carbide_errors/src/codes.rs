#![allow(clippy::zero_prefixed_literal)]

use std::fmt;

pub struct ErrCode(pub u32);

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:04}", self.0)
    }
}

#[macro_export]
#[allow(clippy::zero_prefixed_literal)]
macro_rules! error_codes {
    ($macro:path) => (
        $macro!(
            E0000: 0000, // Lexer::Generic
            E0001: 0001, // Lexer::NonASCIIChar
            E0002: 0002, // Lexer::UnexpectedEOF
            E0003: 0003, // Lexer::UnexpectedChar
            E0004: 0004, // Lexer::UnclosedString
            E0005: 0005, // Lexer::UnmatchedBrace
            E0006: 0006, // Lexer::InvalidNumber

            E1000: 1000, // Parser::Generic
        );
    )
}

macro_rules! define_codes {
    ($($name:ident : $val:expr),* $(,)?) => {
        #[allow(clippy::zero_prefixed_literal)]
        $(pub const $name: ErrCode = ErrCode($val);)*
    };
}

error_codes!(define_codes);
