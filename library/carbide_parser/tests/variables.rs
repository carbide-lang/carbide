#[cfg(test)]
mod variables {
    use carbide_lexer::{
        lexer::CarbideLexer,
        operators::BinaryOperators,
        tokens::{SourceLocation, Token, Tokens},
    };
    use carbide_parser::{
        errors::CarbideParserError,
        nodes::{Expression, LiteralValue, Statement},
        parser::CarbideParser,
    };

    #[test]
    fn declaration_initializer() {
        let src = r#"let my_var = 0;"#;
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::LetDeclaration {
                name: "my_var".to_string(),
                initializer: Some(Expression::Literal(LiteralValue::Int(0)))
            }]
        )
    }

    #[test]
    fn declaration_no_initializer() {
        let src = r#"let my_var;"#;
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::LetDeclaration {
                name: "my_var".to_string(),
                initializer: None,
            }]
        )
    }

    #[test]
    fn invalid_declaration_missing_identifier() {
        let src = r#"let = ;"#;
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();

        assert!(!result.is_ok());

        assert_eq!(
            result.errors,
            vec![Box::new(CarbideParserError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: Token {
                    token_type: Tokens::BinaryOperator(BinaryOperators::Eq),
                    start: SourceLocation {
                        line: 1,
                        column: 5,
                        offset: 4,
                    },
                    end: SourceLocation {
                        line: 1,
                        column: 6,
                        offset: 5
                    },
                    span: 4..5,
                    src: "="
                }
            })]
        )
    }

    #[test]
    fn invalid_declaration_missing_initializer() {
        let src = r#"let my_var = ;"#;
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();

        assert!(!result.is_ok());

        assert_eq!(
            result.errors,
            vec![Box::new(CarbideParserError::UnexpectedToken {
                expected: "expression".to_string(),
                found: Token {
                    token_type: Tokens::Semicolon,
                    start: SourceLocation {
                        line: 1,
                        column: 14,
                        offset: 13,
                    },
                    end: SourceLocation {
                        line: 1,
                        column: 15,
                        offset: 14
                    },
                    span: 13..14,
                    src: ";"
                }
            })]
        )
    }
}
