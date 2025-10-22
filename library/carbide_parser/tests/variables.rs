#[cfg(test)]
mod variables {
    use carbide_lexer::lexer::CarbideLexer;
    use carbide_parser::{
        nodes::{Expression, LiteralValue, Statement, Type},
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
    fn declaration_initializer() {
        let (_, result) = parse_src("let my_var = 0;");
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::LetDeclaration {
                name: "my_var".into(),
                type_annotation: None,
                initializer: Some(Expression::Literal(LiteralValue::Int(0))),
            }]
        );
    }

    #[test]
    fn declaration_no_initializer() {
        let (_, result) = parse_src("let my_var;");
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::LetDeclaration {
                name: "my_var".into(),
                type_annotation: None,
                initializer: None,
            }]
        );
    }

    #[test]
    fn declaration_typed() {
        let (_, result) = parse_src("let my_var: int = 0;");
        assert!(result.is_ok());
        assert_eq!(
            result.ast,
            vec![Statement::LetDeclaration {
                name: "my_var".into(),
                type_annotation: Some(Type::named("int")),
                initializer: Some(Expression::Literal(LiteralValue::Int(0))),
            }]
        );
    }

    #[test]
    fn declaration_typed_no_named() {
        let (_, result) = parse_src("let my_var: = 0;");
        assert!(!result.is_ok());
    }

    #[test]
    fn invalid_missing_identifier() {
        let (_, result) = parse_src("let = 5;");
        assert!(!result.is_ok());
    }

    #[test]
    fn invalid_missing_initializer_value() {
        let (_, result) = parse_src("let my_var = ;");
        assert!(!result.is_ok());
    }

    #[test]
    fn invalid_block_initializer() {
        let (_, result) = parse_src("let my_var = {};");
        assert!(!result.is_ok());
    }

    #[test]
    fn missing_semicolon() {
        let (_, result) = parse_src("let x = 5");
        assert!(!result.is_ok(), "Expected error for missing semicolon");
    }

    #[test]
    fn unterminated_string_literal() {
        let mut lexer = CarbideLexer::from_src(r#"let bad = "oops;"#);
        let result = lexer.lex();
        assert!(
            !result.is_ok(),
            "Unclosed string should trigger lexer error"
        );
    }

    #[test]
    fn extra_whitespace() {
        let (_, result) = parse_src("   let     spaced   =   42   ;   ");
        assert!(result.is_ok());
    }

    #[test]
    fn multiline_declaration() {
        let src = r#"
            let
                foo
                =
                123
                ;
        "#;
        let (_, result) = parse_src(src);
        assert!(result.is_ok());
    }

    #[test]
    fn empty_input() {
        let (_, result) = parse_src("");
        assert!(result.is_ok());
        assert!(result.ast.is_empty());
    }

    #[test]
    fn multiple_declarations_same_line() {
        let src = "let a = 1; let b = 2; let c;";
        let (_, result) = parse_src(src);
        assert!(result.is_ok());
        assert_eq!(result.ast.len(), 3);
    }

    #[test]
    fn multiple_newlines_between_tokens() {
        let src = "let\n\n\nvar\n\n\n=\n\n\n0;";
        let (_, result) = parse_src(src);
        assert!(result.is_ok());
    }

    #[test]
    fn literal_float() {
        let (_, result) = parse_src("let pi = 3.14;");
        assert!(result.is_ok());
    }

    #[test]
    fn literal_string() {
        let (_, result) = parse_src(r#"let s = "text";"#);
        assert!(result.is_ok());
    }

    #[test]
    fn literal_bool() {
        let (_, result) = parse_src("let flag = true;");
        assert!(result.is_ok());
    }

    #[test]
    fn arithmetic_expression() {
        let (_, result) = parse_src("let x = 2 + 3 * 4;");
        assert!(result.is_ok());
    }

    #[test]
    fn nested_parentheses() {
        let (_, result) = parse_src("let x = ((1 + 2) * (3 + 4));");
        assert!(result.is_ok());
    }

    #[test]
    fn chained_binary_operators() {
        let (_, result) = parse_src("let val = 1 + 2 - 3 + 4;");
        assert!(result.is_ok());
    }

    #[test]
    fn missing_rhs_expression() {
        let (_, result) = parse_src("let val = 1 + ;");
        assert!(!result.is_ok());
    }

    #[test]
    fn keyword_as_identifier() {
        let (_, result) = parse_src("let let = 5;");
        assert!(!result.is_ok());
    }

    #[test]
    fn invalid_identifier_character() {
        let (_, result) = parse_src("let my-var = 10;");
        assert!(!result.is_ok(), "Hyphen not allowed in identifiers");
    }

    #[test]
    fn comment_after_declaration() {
        let src = "let a = 10; // trailing comment";
        let (_, result) = parse_src(src);
        assert!(result.is_ok());
    }

    #[test]
    fn comment_between_tokens() {
        let src = "let /* inline comment */ b = 5;";
        let (_, result) = parse_src(src);
        assert!(result.is_ok());
    }

    #[test]
    fn declaration_inside_expression_parens() {
        let (_, result) = parse_src("(let x = 1);");
        assert!(!result.is_ok());
    }

    #[test]
    fn nested_expression_with_unclosed_paren() {
        let (_, result) = parse_src("let a = (1 + (2 * 3);");
        assert!(!result.is_ok(), "Unbalanced parentheses should fail");
    }

    #[test]
    fn long_chain_expression() {
        let (_, result) = parse_src("let z = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9;");
        assert!(result.is_ok(), "Should handle long left-associative chains");
    }
}
