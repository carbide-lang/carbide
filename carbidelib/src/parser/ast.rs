use crate::{errors::ASTError, tokens::Tokens};

pub struct AST {
    pub index: usize,
    pub tokens: Vec<Tokens>,
}

impl TryFrom<Vec<Tokens>> for AST {
    fn try_from(tokens: Vec<Tokens>) -> Result<Self, Self::Error> {
        Ok(Self { index: 0, tokens })
    }

    type Error = ASTError;
}

impl AST {
    pub fn peek(&self) -> Option<&Tokens> {
        return self.tokens.get(self.index + 1);
    }

    pub fn consume_if_eq(&mut self, token: &Tokens) -> bool {
        if let Some(t) = self.peek() {
            if std::mem::discriminant(t) == std::mem::discriminant(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<&Tokens> {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Tokens> {
        if self.index > 0 {
            self.tokens.get(self.index - 1)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
