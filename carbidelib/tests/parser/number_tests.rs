#[cfg(test)]
mod number_tests {
    use carbidelib::{parser::Parser, tokens::Tokens};

    #[test]
    fn valid_int() {
        let mut parser = Parser::from(r#"1230"#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::Integer(1230)], parser.tokens)
    }

    #[test]
    fn valid_neg_int() {
        let mut parser = Parser::from(r#"-1230"#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::Integer(-1230)], parser.tokens)
    }

    #[test]
    fn valid_float() {
        let mut parser = Parser::from(r#"4.2"#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::Float(4.2)], parser.tokens)
    }

    #[test]
    fn valid_neg_float() {
        let mut parser = Parser::from(r#"-2.3"#.to_string());
        parser.parse().expect("Expected parsing to succeed");

        assert_eq!(vec![Tokens::Float(-2.3)], parser.tokens)
    }
}
