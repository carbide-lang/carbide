#[cfg(test)]
pub mod binary {
    use carbide_lexer::{
        lexer::CarbideLexer,
        operators::{BinaryOperators, UnaryOperators},
        tokens::{SourceLocation, Token, Tokens},
    };

    #[test]
    fn all_binary() {
        let src = BinaryOperators::ALL
            .iter()
            .map(|kw| kw.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        let mut lexer = CarbideLexer::from_src(&src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        let mut expected = Vec::new();
        let mut start = 0usize;

        for kw in BinaryOperators::ALL {
            let lit = kw.as_str();
            let len = lit.len();
            let end = start + len;
            expected.push(Token::new(
                Tokens::BinaryOperator(*kw),
                SourceLocation {
                    column: start as u64 + 1,
                    line: 1,
                    offset: start as u64,
                },
                SourceLocation {
                    column: end as u64 + 1,
                    line: 1,
                    offset: end as u64,
                },
                start as u64..end as u64,
                lit,
            ));
            start = end + 1;
        }

        assert_eq!(tokens, expected);
    }

    #[test]
    fn all_unary() {
        let src = UnaryOperators::ALL
            .iter()
            .map(|kw| kw.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        let mut lexer = CarbideLexer::from_src(&src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        let mut expected = Vec::new();
        let mut start = 0usize;

        for kw in UnaryOperators::ALL {
            let lit = kw.as_str();
            let len = lit.len();
            let end = start + len;
            expected.push(Token::new(
                Tokens::UnaryOperator(*kw),
                SourceLocation {
                    column: start as u64 + 1,
                    line: 1,
                    offset: start as u64,
                },
                SourceLocation {
                    column: end as u64 + 1,
                    line: 1,
                    offset: end as u64,
                },
                start as u64..end as u64,
                lit,
            ));
            start = end + 1;
        }

        assert_eq!(tokens, expected);
    }
    #[test]
    fn operator_without_whitespace() {
        let src = "a==b";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, Tokens::Identifier("a"));
        assert_eq!(
            tokens[1].token_type,
            Tokens::BinaryOperator(BinaryOperators::EqEq)
        );
        assert_eq!(tokens[2].token_type, Tokens::Identifier("b"));
    }

    #[test]
    fn ambiguous_operators() {
        let src = "! != !";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0].token_type,
            Tokens::UnaryOperator(UnaryOperators::Not)
        );
        assert_eq!(
            tokens[1].token_type,
            Tokens::BinaryOperator(BinaryOperators::NotEq)
        );
        assert_eq!(
            tokens[2].token_type,
            Tokens::UnaryOperator(UnaryOperators::Not)
        );
    }

    #[test]
    fn operators_adjacent_to_parens() {
        let src = "!(x)";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(tokens.len(), 4);
    }
}
