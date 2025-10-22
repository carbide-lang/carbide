#[cfg(test)]
mod functions {
    use carbide_lexer::lexer::CarbideLexer;
    use carbide_parser::{
        nodes::{Parameter, Statement, Type},
        parser::CarbideParser,
    };

    fn parse_src(src: &'_ str) -> (CarbideParser<'_>, carbide_parser::parser::ParseResult) {
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        assert!(result.is_ok(), "Lexer failed for '{}'", src);

        let mut parser = CarbideParser::new(result.tokens);
        let result = parser.parse();
        (parser, result)
    }

    #[test]
    fn valid_no_params_no_return() {
        let (_, result) = parse_src(r#"fn my_func() {}"#);
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::FunctionDeclaration {
                name: "my_func".to_string(),
                return_type: None,
                parameters: vec![],
                body: vec![]
            }]
        );
    }

    #[test]
    fn valid_no_return() {
        let (_, result) = parse_src(r#"fn add(a: int, b: int) {}"#);
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::FunctionDeclaration {
                name: "add".to_string(),
                return_type: None,
                parameters: vec![
                    Parameter {
                        name: "a".to_string(),
                        type_annotation: Some(Type::named("int")),
                    },
                    Parameter {
                        name: "b".to_string(),
                        type_annotation: Some(Type::named("int"))
                    }
                ],
                body: vec![]
            }]
        );
    }

    #[test]
    fn valid_return_params() {
        let (_, result) = parse_src(r#"fn add(a: int, b: int) -> int {}"#);
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::FunctionDeclaration {
                name: "add".to_string(),
                return_type: Some(Type::named("int")),
                parameters: vec![
                    Parameter {
                        name: "a".to_string(),
                        type_annotation: Some(Type::named("int")),
                    },
                    Parameter {
                        name: "b".to_string(),
                        type_annotation: Some(Type::named("int"))
                    }
                ],
                body: vec![]
            }]
        );
    }
}
