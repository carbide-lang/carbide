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
