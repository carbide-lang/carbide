use crate::{errors::CarbideParserError, tokens::Token};
use logos::{Lexer, Logos};

pub struct CarbideParser<'a> {
    pub src: &'a str,
    pub lexer: Lexer<'a, Token>,
}

impl<'a> CarbideParser<'a> {
    pub fn new(src: &'a str, lexer: Lexer<'a, Token>) -> Self {
        Self { src, lexer }
    }
}

impl<'a> From<&'a str> for CarbideParser<'a> {
    fn from(src: &'a str) -> Self {
        Self {
            src,
            lexer: Token::lexer(src),
        }
    }
}

impl CarbideParser<'_> {
    pub fn parse(&mut self) -> Result<Vec<Token>, CarbideParserError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }
}
