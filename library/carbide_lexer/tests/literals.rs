#[cfg(test)]
pub mod number_literals {
    use carbide_lexer::{
        lexer::CarbideLexer,
        tokens::{Token, Tokens},
    };

    #[test]
    fn valid_int() {
        let src = "100";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::IntLiteral(100), 0..3, "100")]
        )
    }

    #[test]
    fn valid_float() {
        let src = "0.5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::FloatLiteral(0.5), 0..3, "0.5")]
        )
    }

    #[test]
    fn valid_hex() {
        let src = "0xFF";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::HexLiteral(0xFF), 0..4, "0xFF")]
        )
    }

    #[test]
    fn valid_binary() {
        let src = "0b1010";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::BinaryLiteral(0b1010), 0..6, "0b1010")]
        )
    }

    #[test]
    fn empty_hex_literal() {
        let src = "0x";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        assert!(result.is_err(), "Empty hex literal should fail");
    }

    #[test]
    fn empty_binary_literal() {
        let src = "0b";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        assert!(result.is_err(), "Empty binary literal should fail");
    }

    #[test]
    fn multiple_dots_in_float() {
        let src = "1.2.3";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn trailing_dot() {
        let src = "5.";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(tokens[0].token_type, Tokens::FloatLiteral(5.0));
    }

    #[test]
    fn leading_dot() {
        let src = ".5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(tokens[0].token_type, Tokens::Period);
        assert_eq!(tokens[1].token_type, Tokens::IntLiteral(5));
    }

    #[test]
    fn hex_with_invalid_chars() {
        let src = "0xGG";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        assert!(result.is_err())
    }

    #[test]
    fn large_numbers() {
        let src = "9223372036854775807";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(tokens[0].token_type, Tokens::IntLiteral(i64::MAX));
    }

    #[test]
    fn overflow_number() {
        let src = "9223372036854775808";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();
        assert!(result.is_err());
    }

    #[test]
    fn zero_variants() {
        let src = "0 0x0 0b0";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, Tokens::IntLiteral(0));
        assert_eq!(tokens[1].token_type, Tokens::HexLiteral(0));
        assert_eq!(tokens[2].token_type, Tokens::BinaryLiteral(0));
    }
}
