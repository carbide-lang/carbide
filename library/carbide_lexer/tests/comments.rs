// comments.rs - new test file

#[cfg(test)]
pub mod comments {
    use carbide_lexer::errors::CarbideLexerError;
    use carbide_lexer::keywords::Keywords;
    use carbide_lexer::operators::BinaryOperators;
    use carbide_lexer::{lexer::CarbideLexer, tokens::Tokens};

    #[test]
    fn single_line_comment() {
        let src = "let x // this is a comment\n= 5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, Tokens::Keyword(Keywords::Let));
        assert_eq!(tokens[1].token_type, Tokens::Identifier("x"));
    }

    #[test]
    fn single_line_comment_at_end() {
        let src = "let x = 5; // comment";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn multi_line_comment() {
        let src = "let /* this is a\nmulti-line comment */ x = 5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn nested_multi_line_comments_supported() {
        let src = "let /* outer /* inner */ still in outer */ x = 5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer
            .lex()
            .expect("Lexing should succeed with nested comments");

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, Tokens::Keyword(Keywords::Let));
        assert_eq!(tokens[1].token_type, Tokens::Identifier("x"));
    }

    #[test]
    fn deeply_nested_comments() {
        let src = "let /* a /* b /* c */ d */ e */ x = 5";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn unclosed_nested_comment() {
        let src = "let /* a /* b */ c";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex().expect_err("Lexing should fail");

        assert_eq!(result, CarbideLexerError::UnclosedComment);
    }

    #[test]
    fn comment_in_middle_of_expression() {
        let src = "let x = /* comment */ 5 + /* another */ 3";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn only_comments() {
        let src = "// just a comment\n/* and another */";
        let mut lexer = CarbideLexer::from_src(src);
        let tokens = lexer.lex().expect("Lexing should succeed");

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn unclosed_multiline_comment() {
        let src = "let x /* unclosed comment";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex().expect_err("Lexing should fail");

        assert_eq!(result, CarbideLexerError::UnclosedComment)
    }
}
