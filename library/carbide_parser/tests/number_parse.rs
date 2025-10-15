#[cfg(test)]
mod number_parse_tests {
    use carbide_parser::{parser::CarbideParser, tokens::Token};

    #[test]
    fn int_valid_positive() {
        let src = r#"100"#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::Integer(100)]);
    }

    #[test]
    fn int_valid_negative() {
        let src = r#"-100"#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::Integer(-100)]);
    }

    #[test]
    fn float_valid_positive() {
        let src = r#"100.5"#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::Float(100.5)]);
    }

    #[test]
    fn float_valid_negative() {
        let src = r#"-100.5"#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::Float(-100.5)]);
    }
}
