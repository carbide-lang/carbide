#[cfg(test)]
mod integration {
    use carbide_lexer::{lexer::CarbideLexer, operators::BinaryOperators};
    use carbide_parser::{
        nodes::{Expression, LiteralValue, Parameter, Statement, Type},
        parser::CarbideParser,
    };

    fn parse_src(src: &'_ str) -> (CarbideParser<'_>, carbide_parser::parser::ParseResult) {
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        dbg!(&result.errors);
        assert!(result.is_ok(), "Lexer failed for '{}'", src);

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();
        (parser, result)
    }

    #[test]
    fn add() {
        let (_, result) = parse_src(
            r#"
            fn add_int(a: int, b: int) -> int {
                return a + b;
            }

            let c = add_int(4, 2);
        "#,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![
                Statement::FunctionDeclaration {
                    name: "add_int".to_string(),
                    return_type: Some(Type::named("int")),
                    parameters: vec![
                        Parameter {
                            name: "a".to_string(),
                            type_annotation: Some(Type::named("int"))
                        },
                        Parameter {
                            name: "b".to_string(),
                            type_annotation: Some(Type::named("int"))
                        }
                    ],
                    body: vec![Statement::Return(Some(Expression::BinaryOp {
                        left: Box::new(Expression::Identifier("a".to_string())),
                        operator: BinaryOperators::Plus,
                        right: Box::new(Expression::Identifier("b".to_string())),
                    })),],
                },
                Statement::LetDeclaration {
                    name: "c".to_string(),
                    type_annotation: None,
                    initializer: Some(Expression::Call {
                        callee: Box::new(Expression::Identifier("add_int".to_string())),
                        arguments: vec![
                            Expression::Literal(LiteralValue::Int(4)),
                            Expression::Literal(LiteralValue::Int(2))
                        ]
                    })
                }
            ]
        );
    }
}
