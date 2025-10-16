#[cfg(test)]
mod number_parse_tests {
    use carbide_parser::{errors::CarbideParserError, parser::CarbideParser, tokens::Token};

    #[test]
    fn valid_string() {
        let src = r#""Hello World!""#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::String("Hello World!".to_string())]);
    }

    #[test]
    fn valid_string_escaped() {
        let src = r#""\\ \n""#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::String("\\ \n".to_string())]);
    }

    #[test]
    fn invalid_string_extra_quote() {
        let src = r#" """ "#;
        let mut parser = CarbideParser::from(src);
        let result = parser.parse().expect_err("Parse should be invalid");
        assert_eq!(result, CarbideParserError::UnexpectedChar('"'));
    }

    #[test]
    fn invalid_string_extra_single_quote() {
        let src = r#" "'" "#;
        let mut parser = CarbideParser::from(src);
        let result = parser.parse().expect("Parse should be valid");
        assert_eq!(result, vec![Token::String("'".to_string())]);
    }
}
