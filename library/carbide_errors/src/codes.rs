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
            E1001: 1001, // Parser::UnexpectedEOF
            E1002: 1002, // Parser::UnexpectedToken
            E1010: 1010, // Parser::ExpectedIdentifier
            E1011: 1011, // Parser::ExpectedExpression
            E1020: 1020, // Parser::TooManyParameters
            E1021: 1021, // Parser::TooManyArguments
            E1030: 1030, // Parser::InvalidAssignmentTarget
            E1040: 1040, // Parser::BreakOutsideLoop
            E1041: 1041, // Parser::ContinueOutsideLoop
            E1042: 1042, // Parser::ReturnOutsideFunction
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
