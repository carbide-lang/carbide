#[cfg(test)]
pub mod number_literals {
    use carbide_parser::{parser::CarbideParser, tokens::Token};

    #[test]
    fn valid_int() {
        let src = "100";
        let mut parser = CarbideParser::from_src(src);
        let tokens = parser.parse_tokens().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(
                carbide_parser::tokens::Tokens::IntLiteral(100),
                0..3,
                "100"
            )]
        )
    }

    #[test]
    fn valid_float() {
        let src = "0.5";
        let mut parser = CarbideParser::from_src(src);
        let tokens = parser.parse_tokens().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(
                carbide_parser::tokens::Tokens::FloatLiteral(0.5),
                0..3,
                "0.5"
            )]
        )
    }

    #[test]
    fn valid_hex() {
        let src = "0xFF";
        let mut parser = CarbideParser::from_src(src);
        let tokens = parser.parse_tokens().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(
                carbide_parser::tokens::Tokens::HexLiteral(0xFF),
                0..4,
                "0xFF"
            )]
        )
    }
}
