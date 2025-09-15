pub mod ast;
pub mod expr;
pub mod nodes;
pub mod ops;

use crate::{errors::ParserError, tokens::Tokens};
use logos::Logos;

pub struct Parser {
    pub src: String,
    pub tokens: Vec<Tokens>,
}

impl Parser {
    pub fn parse(&mut self) -> Result<(), ParserError> {
        let mut lexer = Tokens::lexer(&self.src);

        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            match token {
                Ok(tok) => tokens.push(tok),
                Err(_) => {
                    return Err(ParserError::InvalidToken(
                        lexer.slice().to_owned(),
                        lexer.span().start,
                        lexer.span().end,
                    ));
                }
            }
        }

        self.tokens = tokens;
        Ok(())
    }
}

impl From<String> for Parser {
    fn from(src: String) -> Self {
        Self {
            src,
            tokens: Vec::new(),
        }
    }
}
