#[cfg(test)]
pub mod identifier {
    use carbide_lexer::{
        lexer::CarbideLexer,
        tokens::{SourceLocation, Token, Tokens},
    };

    #[test]
    fn valid_snakecase() {
        let src = "my_ident";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("my_ident"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 9,
                    offset: 8
                },
                0..8,
                "my_ident"
            )]
        )
    }

    #[test]
    fn valid_camelcase() {
        let src = "myIdent";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("myIdent"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 8,
                    offset: 7
                },
                0..7,
                "myIdent"
            )]
        )
    }

    #[test]
    fn valid_pascalcase() {
        let src = "MyIdent";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("MyIdent"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 8,
                    offset: 7
                },
                0..7,
                "MyIdent"
            )]
        )
    }

    #[test]
    fn valid_constcase() {
        let src = "MY_IDENT";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("MY_IDENT"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 9,
                    offset: 8
                },
                0..8,
                "MY_IDENT"
            )]
        )
    }

    #[test]
    fn number_prefix() {
        let src = "0ident";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    Tokens::IntLiteral(0),
                    SourceLocation {
                        line: 1,
                        column: 1,
                        offset: 0
                    },
                    SourceLocation {
                        line: 1,
                        column: 2,
                        offset: 1
                    },
                    0..1,
                    "0"
                ),
                Token::new(
                    Tokens::Identifier("ident"),
                    SourceLocation {
                        line: 1,
                        column: 2,
                        offset: 1
                    },
                    SourceLocation {
                        line: 1,
                        column: 7,
                        offset: 6
                    },
                    1..6,
                    "ident"
                )
            ]
        )
    }

    #[test]
    fn number_suffix() {
        let src = "ident0";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("ident0"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 7,
                    offset: 6
                },
                0..6,
                "ident0"
            )]
        )
    }

    #[test]
    fn underscore_prefix() {
        let src = "_ident";
        let mut lexer = CarbideLexer::from_src(src);
        let result = lexer.lex();

        assert!(result.is_ok());
        let tokens = result.tokens;

        assert_eq!(
            tokens,
            vec![Token::new(
                Tokens::Identifier("_ident"),
                SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                SourceLocation {
                    line: 1,
                    column: 7,
                    offset: 6
                },
                0..6,
                "_ident"
            )]
        )
    }
}

#[cfg(test)]
pub mod keyword {
    use carbide_lexer::{
        keywords::Keywords,
        lexer::CarbideLexer,
        tokens::{SourceLocation, Token, Tokens},
    };

    #[test]
    fn all_keywords() {
        let src = Keywords::ALL
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

        for kw in Keywords::ALL {
            let lit = kw.as_str();
            let len = lit.len();
            let end = start + len;
            expected.push(Token::new(
                Tokens::Keyword(*kw),
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
}
