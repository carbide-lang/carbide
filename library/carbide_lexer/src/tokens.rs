use std::fmt;
use std::ops::Range;

use crate::keywords::Keywords;
use crate::operators::{BinaryOperators, UnaryOperators};

/// Represents a location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    pub line: u64,
    pub column: u64,
    pub offset: u64,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    /// Literal text within the string
    Text(String),
    /// An interpolation placeholder like `{name}`
    Interpolation(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens<'a> {
    /// Integer literal, like `100`
    IntLiteral(i64),
    /// Float literal, like `0.5`
    FloatLiteral(f64),
    /// A hexadecimal literal, like `0xFF`
    HexLiteral(i64),
    /// A binary literal, like `0b1010`
    BinaryLiteral(i64),
    /// A string literal, like `"Hello World"`
    StringLiteral(String),
    /// An interpolated string with `{}`, like `"Hello {name}"`
    InterpolatedString(Vec<StringPart>),
    /// An indentifier, like `my_ident`
    Identifier(&'a str),
    /// A keyword, like `let` or `fn`
    Keyword(Keywords),
    /// A binary operator, like `!=`
    BinaryOperator(BinaryOperators),
    /// A unary operator, like `!`
    UnaryOperator(UnaryOperators),
    /// A type identifier, like `string`
    TypeIdentifier(&'a str),

    ThinArrow,
    FatArrow,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Period,
    Comma,
    Tilde,
}

macro_rules! define_single_char_tokens {
    ($($variant:ident => $char:literal),* $(,)?) => {
        impl Tokens<'_> {
            /// Attempt to parse a single char token
            #[must_use]
            pub fn from_char(ch: char) -> Option<Self> {
                match ch {
                    $($char => Some(Self::$variant),)*
                    _ => None,
                }
            }

            /// Returns `true` if the char can start a char token
            #[must_use]
            pub fn starts_with(ch: char) -> bool {
                match ch {
                    $($char => true,)*
                    _ => false,
                }
            }
        }
    };
}

define_single_char_tokens! {
    LeftParen => '(',
    RightParen => ')',
    LeftBracket => '[',
    RightBracket => ']',
    LeftBrace => '{',
    RightBrace => '}',
    Semicolon => ';',
    Colon => ':',
    Period => '.',
    Comma => ',',
    Tilde => '~',
}

pub type Span = Range<u64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: Tokens<'a>,
    pub start: SourceLocation,
    pub end: SourceLocation,
    pub span: Span,
    pub src: &'a str,
}

impl<'a> Token<'a> {
    #[must_use]
    pub fn new(
        token_type: Tokens<'a>,
        start: SourceLocation,
        end: SourceLocation,
        span: Span,
        src: &'a str,
    ) -> Self {
        Self {
            token_type,
            start,
            end,
            span,
            src,
        }
    }

    /// Get the line number where this token starts
    #[must_use]
    pub fn line(&self) -> u64 {
        self.start.line
    }

    /// Get the column number where this token starts
    #[must_use]
    pub fn column(&self) -> u64 {
        self.start.column
    }

    /// Get a formatted location string (line:column)
    #[must_use]
    pub fn location_str(&self) -> String {
        format!("{}:{}", self.start.line, self.start.column)
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Tokens::{:?}@{}..{} `{}`>",
            self.token_type, self.start, self.end, self.src
        )
    }
}
