use crate::{
    errors::{ASTError, ParserError},
    parser::{
        expr::Expr,
        nodes::Nodes,
        ops::{BinaryOp, UnaryOp},
    },
    tokens::Tokens,
    types::Types,
};

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

impl AST {
    pub fn construct(&mut self) -> Result<Vec<Expr>, ASTError> {
        let mut statements = vec![];

        loop {
            if self.is_at_end() {
                break;
            }

            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Expr, ASTError> {
        let expr = self.expression()?;

        self.consume_if_eq(&Tokens::Semicolon);

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ASTError> {
        // Keyword specific parsing

        self.assignment()
    }

    fn consume_type(&mut self) -> Result<Types, ASTError> {
        if let Some(token) = self.peek().cloned() {
            return match token {
                Tokens::Identifier(ident) => {
                    Ok(Types::try_from(ident).map_err(|e| ASTError::ParserError(e.to_string()))?)
                }
                _ => Err(ASTError::SyntaxError(
                    "Invalid type".to_string(),
                    token.to_string(),
                )),
            };
        }
        Err(ASTError::UnexpectedEOF(
            self.previous()
                .unwrap_or(&Tokens::String("NUL".to_string()))
                .to_string(),
        ))
    }

    fn assignment(&mut self) -> Result<Expr, ASTError> {
        // Declaration
        if self.consume_if_eq(&Tokens::Let) {
            if let Some(Tokens::Identifier(ident)) = self.peek().cloned() {
                self.advance(); // Consume identifier

                let var_type = if let Some(Tokens::Tilde) = self.peek().cloned() {
                    Some(self.consume_type()?)
                } else {
                    None
                };

                let value = self.assignment()?;

                return Ok(Expr::Declaration {
                    identifier: ident,
                    var_type,
                    value: Box::from(value),
                });
            }
        }

        let expr = self.or()?;

        if self.consume_if_eq(&Tokens::Equals) {
            if let Expr::Literal(Nodes::Identifier(ident)) = expr {
                let value = self.assignment()?;

                return Ok(Expr::Assignment {
                    identifier: ident,
                    value: Box::from(value),
                });
            }

            return Err(ASTError::AssignmentError(
                "Cannot assign to non-identifier target".to_string(),
            ));
        }

        Ok(expr)
    }
}

impl AST {
    fn or(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.and()?;

        while self.consume_if_eq(&Tokens::Or) {
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(BinaryOp::Or),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.equality()?;

        while self.consume_if_eq(&Tokens::And) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(BinaryOp::And),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.match_equality_op() {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.term()?;

        while let Some(op) = self.match_comparison_op() {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
}

impl AST {
    pub fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.consume_if_eq(&Tokens::NotEquals) {
            Some(BinaryOp::NotEqual)
        } else if self.consume_if_eq(&Tokens::EqualsEquals) {
            Some(BinaryOp::Equal)
        } else {
            None
        }
    }

    pub fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.consume_if_eq(&Tokens::Greater) {
            Some(BinaryOp::Greater)
        } else if self.consume_if_eq(&Tokens::GreaterOrEq) {
            Some(BinaryOp::GreaterEqual)
        } else if self.consume_if_eq(&Tokens::Less) {
            Some(BinaryOp::Less)
        } else if self.consume_if_eq(&Tokens::LessOrEq) {
            Some(BinaryOp::LessEqual)
        } else {
            None
        }
    }

    pub fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.consume_if_eq(&Tokens::Minus) {
            Some(BinaryOp::Subtract)
        } else if self.consume_if_eq(&Tokens::Plus) {
            Some(BinaryOp::Add)
        } else {
            None
        }
    }

    pub fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.consume_if_eq(&Tokens::Slash) {
            Some(BinaryOp::Divide)
        } else if self.consume_if_eq(&Tokens::Star) {
            Some(BinaryOp::Multiply)
        } else if self.consume_if_eq(&Tokens::Percent) {
            Some(BinaryOp::Modulo)
        } else {
            None
        }
    }

    pub fn match_unary_op(&mut self) -> Option<UnaryOp> {
        if self.consume_if_eq(&Tokens::Minus) {
            Some(UnaryOp::Minus)
        } else if self.consume_if_eq(&Tokens::Bang) {
            Some(UnaryOp::Not)
        } else {
            None
        }
    }
}

impl AST {
    pub fn term(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.factor()?;

        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr, ASTError> {
        let mut expr = self.unary()?;

        while let Some(op) = self.match_factor_op() {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Box::new(op),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, ASTError> {
        if let Some(op) = self.match_unary_op() {
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                operator: Box::new(op),
                operand: Box::new(expr),
            });
        }

        // if we implement pointers, we should here
        
        Err(ASTError::UnexpectedEOI(
            self.peek().cloned().unwrap_or(Tokens::NUL).to_string(),
        ))
    }
}
