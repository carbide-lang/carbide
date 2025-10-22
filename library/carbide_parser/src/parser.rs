use carbide_lexer::keywords::Keywords;
use carbide_lexer::operators::BinaryOperators;
use carbide_lexer::tokens::{SourceLocation, Token, Tokens};

use crate::errors::CarbideParserError;
use crate::nodes::{Expression, LiteralValue, Parameter, Statement, StringPart, Type};

pub struct CarbideParser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

/// Result type for parsing
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub errors: Vec<Box<CarbideParserError>>,
}

impl ParseResult {
    /// Check if parsing succeeded without errors
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there were any errors
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl<'a> CarbideParser<'a> {
    #[must_use]
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Check if we're at the end of the input
    #[inline]
    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Peek at the current [`Token`] without consuming it
    #[inline]
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    #[inline]
    fn last(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos - 1)
    }

    /// Peek ahead by `n` tokens
    #[inline]
    #[allow(dead_code)]
    fn peek_ahead(&self, n: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.pos + n)
    }

    /// Consume and return the current token
    #[inline]
    fn advance(&mut self) -> Option<&Token<'a>> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Get current source location for error reporting
    fn current_location(&self) -> SourceLocation {
        self.peek()
            .map_or(unsafe { self.last().unwrap_unchecked().end }, |i| i.end)
    }

    /// Check if current token matches a specific token type pattern
    fn check(&self, pattern: impl Fn(&Tokens) -> bool) -> bool {
        if let Some(token) = self.peek() {
            pattern(&token.token_type)
        } else {
            false
        }
    }

    /// Consume token if it matches pattern
    fn match_token(&mut self, pattern: impl Fn(&Tokens) -> bool) -> bool {
        if self.check(pattern) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Expect a specific [`Token`] and consume it
    ///
    /// # Errors
    /// Returns `Err` if EOF is found instead of a token
    fn expect(
        &mut self,
        pattern: impl Fn(&Tokens) -> bool,
        expected: &str,
    ) -> Result<&Token<'a>, Box<CarbideParserError>> {
        if let Some(token) = self.peek() {
            if pattern(&token.token_type) {
                Ok(unsafe { self.advance().unwrap_unchecked() })
            } else {
                Err(Box::new(CarbideParserError::UnexpectedToken {
                    expected: expected.to_string(),
                    found: unsafe { std::mem::transmute::<Token<'_>, Token<'_>>(token.clone()) },
                }))
            }
        } else {
            Err(Box::new(CarbideParserError::UnexpectedEOF(
                self.current_location(),
            )))
        }
    }

    /// Synchronize parser state after an error by advancing to next statement
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_eof() {
            if let Some(prev) = self.tokens.get(self.pos.saturating_sub(1))
                && matches!(prev.token_type, Tokens::Semicolon)
            {
                return;
            }

            if let Some(token) = self.peek()
                && let Tokens::Keyword(kw) = &token.token_type
            {
                match kw {
                    Keywords::Fn | Keywords::Let | Keywords::Return => return,
                }
            }

            self.advance();
        }
    }

    /// Parse tokens into an AST with error recovery
    pub fn parse(&mut self) -> ParseResult {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_eof() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.synchronize();
                }
            }
        }

        ParseResult {
            ast: statements,
            errors,
        }
    }

    /// Strict parsing that fails on first error
    ///
    /// # Errors
    /// Returns `Err` if parsing fails
    pub fn parse_strict(&mut self) -> Result<Vec<Statement>, Box<CarbideParserError>> {
        let result = self.parse();

        if let Some(first_error) = result.errors.into_iter().next() {
            Err(first_error)
        } else {
            Ok(result.ast)
        }
    }
}

impl CarbideParser<'_> {
    /// Attempt to parse a type annotation
    ///
    /// # Errors
    /// Returns `Err` if parsing the type fails
    fn parse_type(&mut self) -> Result<Type, Box<CarbideParserError>> {
        if let Some(token) = self.peek() {
            match &token.token_type {
                Tokens::TypeIdentifier(name) | Tokens::Identifier(name) => {
                    let type_name = (*name).to_string();
                    self.advance();
                    Ok(Type::Named(type_name))
                }
                Tokens::LeftBracket => {
                    self.advance();
                    let element_type = self.parse_type()?;
                    self.expect(|t| matches!(t, Tokens::RightBracket), "]")?;
                    Ok(Type::Array(Box::new(element_type)))
                }
                _ => Err(Box::new(CarbideParserError::UnexpectedToken {
                    expected: "type".to_string(),
                    found: unsafe { std::mem::transmute::<Token<'_>, Token<'_>>(token.clone()) },
                })),
            }
        } else {
            Err(Box::new(CarbideParserError::UnexpectedEOF(
                self.current_location(),
            )))
        }
    }

    /// Attempt to parse a [`Statement`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_statement(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        if let Some(token) = self.peek() {
            match &token.token_type {
                Tokens::Keyword(Keywords::Let) => self.parse_let_statement(),
                Tokens::Keyword(Keywords::Fn) => self.parse_function_declaration(),
                Tokens::Keyword(Keywords::Return) => self.parse_return(),
                Tokens::LeftBrace => self.parse_block_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(Box::new(CarbideParserError::UnexpectedEOF(
                self.current_location(),
            )))
        }
    }

    /// Attempt to parse a `let` [`Statement`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_let_statement(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        self.expect(|t| matches!(t, Tokens::Keyword(Keywords::Let)), "let")?;

        let name_token = self.expect(|t| matches!(t, Tokens::Identifier(_)), "identifier")?;

        let name = if let Tokens::Identifier(n) = &name_token.token_type {
            (*n).to_string()
        } else {
            return Err(Box::new(CarbideParserError::ExpectedIdentifier(unsafe {
                std::mem::transmute::<Token<'_>, Token<'_>>(name_token.clone())
            })));
        };

        // Parse optional type annotation
        let type_annotation = if self.match_token(|t| matches!(t, Tokens::Colon)) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer =
            if self.match_token(|t| matches!(t, Tokens::BinaryOperator(BinaryOperators::Eq))) {
                match self.parse_expression() {
                    Ok(expr) => Some(expr),
                    Err(_) => {
                        return Err(Box::new(CarbideParserError::InvalidAssignmentTarget(
                            self.current_location(),
                        )));
                    }
                }
            } else {
                None
            };

        self.expect(|t| matches!(t, Tokens::Semicolon), ";")?;

        Ok(Statement::LetDeclaration {
            name,
            type_annotation,
            initializer,
        })
    }

    /// Attempt to parse a block [`Statement`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_block_statement(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        self.expect(|t| matches!(t, Tokens::LeftBrace), "{")?;

        let mut statements = Vec::new();

        while !self.is_eof() && !self.check(|t| matches!(t, Tokens::RightBrace)) {
            statements.push(self.parse_statement()?);
        }

        self.expect(|t| matches!(t, Tokens::RightBrace), "}")?;

        Ok(Statement::Block(statements))
    }

    /// Attempt to parse an expression [`Statement`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_expression_statement(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        let expr = self.parse_expression()?;
        self.expect(|t| matches!(t, Tokens::Semicolon), ";")?;
        Ok(Statement::Expression(expr))
    }

    /// Attempt to parse an [`Expression`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_expression(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        self.parse_assignment()
    }

    /// Attempt to parse an assignment
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_assignment(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        let expr = self.parse_equality()?;

        if self.match_token(|t| matches!(t, Tokens::BinaryOperator(BinaryOperators::Eq))) {
            let value = self.parse_assignment()?;
            return Ok(Expression::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }

        Ok(expr)
    }

    /// Attempt to parse an equality expression
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_equality(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            if let Tokens::BinaryOperator(op) = &token.token_type {
                match op {
                    BinaryOperators::EqEq | BinaryOperators::NotEq => {
                        let operator = *op;
                        self.advance();
                        let right = self.parse_comparison()?;
                        left = Expression::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        };
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Attempt to parse a comparison
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_comparison(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        self.parse_term()
    }

    /// Attempt to parse a term
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_term(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            if let Tokens::BinaryOperator(op) = &token.token_type {
                match op {
                    BinaryOperators::Plus | BinaryOperators::Minus => {
                        let operator = *op;
                        self.advance();
                        let right = self.parse_factor()?;
                        left = Expression::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        };
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Attempt to parse a binary operator
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_factor(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.peek() {
            if let Tokens::BinaryOperator(op) = &token.token_type {
                match op {
                    BinaryOperators::Star | BinaryOperators::Slash => {
                        let operator = *op;
                        self.advance();
                        let right = self.parse_unary()?;
                        left = Expression::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        };
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Attempt to parse a unary operation
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_unary(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        if let Some(token) = self.peek()
            && let Tokens::UnaryOperator(op) = &token.token_type
        {
            let operator = *op;
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp {
                operator,
                operand: Box::new(expr),
            });
        }

        self.parse_call()
    }
}

impl CarbideParser<'_> {
    /// Attempt to parse a function declaration
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_function_declaration(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        self.expect(|t| matches!(t, Tokens::Keyword(Keywords::Fn)), "fn")?;

        let name_token = self.expect(|t| matches!(t, Tokens::Identifier(_)), "identifier")?;

        let name = if let Tokens::Identifier(n) = &name_token.token_type {
            (*n).to_string()
        } else {
            return Err(Box::new(CarbideParserError::ExpectedIdentifier(unsafe {
                std::mem::transmute::<Token<'_>, Token<'_>>(name_token.clone())
            })));
        };

        self.expect(|t| matches!(t, Tokens::LeftParen), "(")?;

        let mut parameters = Vec::new();
        if !self.check(|t| matches!(t, Tokens::RightParen)) {
            loop {
                let param_token =
                    self.expect(|t| matches!(t, Tokens::Identifier(_)), "parameter name")?;

                let param_name = if let Tokens::Identifier(param) = &param_token.token_type {
                    (*param).to_string()
                } else {
                    return Err(Box::new(CarbideParserError::ExpectedIdentifier(unsafe {
                        std::mem::transmute::<Token<'_>, Token<'_>>(param_token.clone())
                    })));
                };

                // Parse optional type annotation for parameter
                let type_annotation = if self.match_token(|t| matches!(t, Tokens::Colon)) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                });

                if !self.match_token(|t| matches!(t, Tokens::Comma)) {
                    break;
                }
            }
        }

        self.expect(|t| matches!(t, Tokens::RightParen), ")")?;

        // Parse optional return type annotation
        let return_type = if self.match_token(|t| matches!(t, Tokens::ThinArrow)) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = if self.check(|t| matches!(t, Tokens::LeftBrace)) {
            if let Statement::Block(stmts) = self.parse_block_statement()? {
                stmts
            } else {
                Vec::new()
            }
        } else {
            return Err(Box::new(CarbideParserError::UnexpectedToken {
                expected: "function body".to_string(),
                found: unsafe {
                    std::mem::transmute::<Token<'_>, Token<'_>>(
                        self.peek().unwrap_unchecked().clone(),
                    )
                },
            }));
        };

        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    /// Attempt to parse a function call
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_call(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(|t| matches!(t, Tokens::LeftParen)) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(|t| matches!(t, Tokens::LeftBracket)) {
                let index = self.parse_expression()?;
                self.expect(|t| matches!(t, Tokens::RightBracket), "]")?;
                expr = Expression::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(|t| matches!(t, Tokens::Period)) {
                let member_token =
                    self.expect(|t| matches!(t, Tokens::Identifier(_)), "property name")?;

                if let Tokens::Identifier(name) = &member_token.token_type {
                    expr = Expression::MemberAccess {
                        target: Box::new(expr),
                        member: (*name).to_string(),
                    };
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }
}

impl CarbideParser<'_> {
    /// Attempt to parse a function call
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, Box<CarbideParserError>> {
        let mut arguments = Vec::new();

        if !self.check(|t| matches!(t, Tokens::RightParen)) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_token(|t| matches!(t, Tokens::Comma)) {
                    break;
                }
            }
        }

        self.expect(|t| matches!(t, Tokens::RightParen), ")")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    /// Attempt to parse a primary [`Expression`]
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_primary(&mut self) -> Result<Expression, Box<CarbideParserError>> {
        if let Some(token) = self.peek() {
            match &token.token_type {
                Tokens::FloatLiteral(val) => {
                    let value = *val;
                    self.advance();
                    Ok(Expression::Literal(LiteralValue::Float(value)))
                }
                Tokens::StringLiteral(s) => {
                    let value = s.clone();
                    self.advance();
                    Ok(Expression::Literal(LiteralValue::String(value)))
                }
                Tokens::IntLiteral(val) | Tokens::HexLiteral(val) | Tokens::BinaryLiteral(val) => {
                    let value = *val;
                    self.advance();
                    Ok(Expression::Literal(LiteralValue::Int(value)))
                }
                Tokens::InterpolatedString(parts) => {
                    let string_parts = self.parse_interpolated_string(parts)?;
                    self.advance();
                    Ok(Expression::InterpolatedString {
                        parts: string_parts,
                    })
                }
                Tokens::Identifier(name) => {
                    let ident = (*name).to_string();
                    self.advance();

                    if ident == "true" || ident == "false" {
                        return Ok(Expression::Literal(LiteralValue::Bool(ident == "true")));
                    }

                    Ok(Expression::Identifier(ident))
                }
                Tokens::LeftParen => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.expect(|t| matches!(t, Tokens::RightParen), ")")?;
                    Ok(Expression::Grouped(Box::new(expr)))
                }
                Tokens::LeftBracket => {
                    self.advance();
                    let mut elements = Vec::new();

                    if !self.check(|t| matches!(t, Tokens::RightBracket)) {
                        loop {
                            elements.push(self.parse_expression()?);
                            if !self.match_token(|t| matches!(t, Tokens::Comma)) {
                                break;
                            }
                        }
                    }

                    self.expect(|t| matches!(t, Tokens::RightBracket), "]")?;
                    Ok(Expression::Array(elements))
                }
                _ => Err(Box::new(CarbideParserError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: unsafe { std::mem::transmute::<Token<'_>, Token<'_>>(token.clone()) },
                })),
            }
        } else {
            Err(Box::new(CarbideParserError::UnexpectedEOF(
                self.current_location(),
            )))
        }
    }

    /// Attempt to parse an interpolated string
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    fn parse_interpolated_string(
        &self,
        parts: &[carbide_lexer::tokens::StringPart],
    ) -> Result<Vec<StringPart>, Box<CarbideParserError>> {
        let mut result = Vec::new();

        for part in parts {
            match part {
                carbide_lexer::tokens::StringPart::Text(text) => {
                    result.push(StringPart::Text(text.clone()));
                }
                carbide_lexer::tokens::StringPart::Interpolation(code) => {
                    let mut lexer = carbide_lexer::lexer::CarbideLexer::from_src(code);
                    let tokens = lexer.lex_strict().map_err(|_| {
                        CarbideParserError::ExpectedExpression(self.current_location())
                    })?;

                    let mut mini_parser = CarbideParser::new(tokens);
                    let expr = mini_parser.parse_expression()?;
                    result.push(StringPart::Expression(Box::new(expr)));
                }
            }
        }

        Ok(result)
    }
}

impl CarbideParser<'_> {
    /// Attempt to parse a return statement
    ///
    /// # Errors
    /// Returns `Err` if parsing the tokens fail
    pub fn parse_return(&mut self) -> Result<Statement, Box<CarbideParserError>> {
        self.expect(|t| matches!(t, Tokens::Keyword(Keywords::Return)), "return")?;

        let return_expr = self.parse_expression()?;
        self.expect(|t| matches!(t, Tokens::Semicolon), ";")?;

        Ok(Statement::Return(Some(return_expr)))
    }
}
