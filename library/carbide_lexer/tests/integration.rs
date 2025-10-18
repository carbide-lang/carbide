#[cfg(test)]
pub mod integration {
    use carbide_lexer::{
        errors::CarbideLexerError, keywords::Keywords, lexer::CarbideLexer, tokens::SourceLocation,
        tokens::Tokens,
    };

    #[test]
    fn simple_function_declaration() {
        let src = "fn main() {}";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens[0].token_type, Tokens::Keyword(Keywords::Fn));
        assert_eq!(tokens[1].token_type, Tokens::Identifier("main"));
        assert_eq!(tokens[2].token_type, Tokens::LeftParen);
        assert_eq!(tokens[3].token_type, Tokens::RightParen);
        assert_eq!(tokens[4].token_type, Tokens::LeftBrace);
        assert_eq!(tokens[5].token_type, Tokens::RightBrace);
    }

    #[test]
    fn variable_declaration_with_comparison() {
        let src = "let x = 5; let y = x == 10;";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert!(tokens.len() > 0);
    }

    #[test]
    fn nested_brackets() {
        let src = "[[{}]]";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn mixed_number_types() {
        let src = "let a = 42; let b = 3.14; let c = 0xFF; let d = 0b1010;";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        let int_count = tokens
            .iter()
            .filter(|t| matches!(t.token_type, Tokens::IntLiteral(_)))
            .count();
        let float_count = tokens
            .iter()
            .filter(|t| matches!(t.token_type, Tokens::FloatLiteral(_)))
            .count();
        let hex_count = tokens
            .iter()
            .filter(|t| matches!(t.token_type, Tokens::HexLiteral(_)))
            .count();
        let bin_count = tokens
            .iter()
            .filter(|t| matches!(t.token_type, Tokens::BinaryLiteral(_)))
            .count();

        assert_eq!(int_count, 1);
        assert_eq!(float_count, 1);
        assert_eq!(hex_count, 1);
        assert_eq!(bin_count, 1);
    }

    #[test]
    fn empty_source() {
        let src = "";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn only_whitespace() {
        let src = "   \n\t\r\n  ";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn unicode_in_identifier() {
        let src = "café";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(!result.is_ok());
        assert!(result.has_errors());
    }
}
