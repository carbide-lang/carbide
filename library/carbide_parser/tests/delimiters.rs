#[cfg(test)]
mod delimiters {
    use carbide_lexer::lexer::CarbideLexer;
    use carbide_parser::{
        nodes::{Expression, LiteralValue, Statement},
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
    fn block() {
        let (_, result) = parse_src("{ let my_var = 0; }");
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::Block(vec![Statement::LetDeclaration {
                name: "my_var".into(),
                initializer: Some(Expression::Literal(LiteralValue::Int(0))),
                type_annotation: None
            }])]
        );
    }

    #[test]
    fn empty_block() {
        let (_, result) = parse_src("{  }");
        assert!(result.is_ok());
        assert_eq!(result.ast, vec![Statement::Block(vec![])]);
    }

    #[test]
    fn valid_array() {
        let (_, result) = parse_src("[0,0,0,0];");

        assert!(result.is_ok());

        assert_eq!(
            result.ast,
            vec![Statement::Expression(Expression::Array(vec![
                Expression::Literal(LiteralValue::Int(0)),
                Expression::Literal(LiteralValue::Int(0)),
                Expression::Literal(LiteralValue::Int(0)),
                Expression::Literal(LiteralValue::Int(0))
            ]))]
        );
    }
}
