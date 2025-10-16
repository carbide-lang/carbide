#[cfg(test)]
mod number_literals {
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

#[cfg(test)]
mod string_literals {
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
        let src = r#" "\\ \n" "#;
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

#[cfg(test)]
mod bool_literals {
    use carbide_parser::{parser::CarbideParser, tokens::Token};

    #[test]
    fn valid_bools() {
        let src = r#"true false"#;
        let mut parser = CarbideParser::from(src);
        let tokens = parser.parse().expect("Parse should be valid");
        assert_eq!(tokens, vec![Token::Boolean(true), Token::Boolean(false)]);
    }
}
