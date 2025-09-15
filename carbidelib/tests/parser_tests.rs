#[cfg(test)]
mod parser_test {
    use carbidelib::{parser::Parser, tokens::Tokens};

    #[test]
    fn valid_string() {
        let mut parser = Parser::from(r#""Hello World!""#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(
            vec![Tokens::String("Hello World!".to_string())],
            parser.tokens
        )
    }

    #[test]
    fn valid_string_newline() {
        let mut parser = Parser::from(r#""Hello \n""#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::String("Hello \n".to_string())], parser.tokens)
    }

    #[test]
    fn valid_string_escaped_quote() {
        let mut parser = Parser::from(r#""Hello \"""#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::String("Hello \"".to_string())], parser.tokens)
    }

    #[test]
    fn invalid_string() {
        let mut parser = Parser::from(r#""Hello "#.to_string());
        parser.parse().expect_err("Expected parsing to fail");

        assert!(parser.tokens.is_empty())
    }

    #[test]
    fn invalid_string_escape() {
        let mut parser = Parser::from(r#""Hello \""#.to_string());
        parser.parse().expect_err("Expected parsing to fail");

        assert!(parser.tokens.is_empty())
    }
}
