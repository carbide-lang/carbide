#[cfg(test)]
pub mod identifier {
    use carbide_lexer::{
        lexer::CarbideLexer,
        tokens::{Token, Tokens},
    };

    #[test]
    fn valid_snakecase() {
        let src = "my_ident";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("my_ident"), 0..8, "my_ident")]
        )
    }

    #[test]
    fn valid_camelcase() {
        let src = "myIdent";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("myIdent"), 0..7, "myIdent")]
        )
    }

    #[test]
    fn valid_pascalcase() {
        let src = "MyIdent";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("MyIdent"), 0..7, "MyIdent")]
        )
    }

    #[test]
    fn valid_constcase() {
        let src = "My_IDENT";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("My_IDENT"), 0..8, "My_IDENT")]
        )
    }

    #[test]
    fn number_prefix() {
        let src = "0ident";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![
                Token::new(Tokens::IntLiteral(0), 0..1, "0"),
                Token::new(Tokens::Identifier("ident"), 1..6, "ident")
            ]
        )
    }

    #[test]
    fn number_suffix() {
        let src = "ident0";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("ident0"), 0..6, "ident0")]
        )
    }

    #[test]
    fn underscore_prefix() {
        let src = "_ident";
        let mut parser = CarbideLexer::from_src(src);
        let tokens = parser.lex().expect("Parsing should succeed");
        assert_eq!(
            tokens,
            vec![Token::new(Tokens::Identifier("_ident"), 0..6, "_ident")]
        )
    }
}

#[cfg(test)]
pub mod keyword {
    use carbide_lexer::{
        keywords::Keywords,
        lexer::CarbideLexer,
        tokens::{Token, Tokens},
    };

    #[test]
    fn all_keywords() {
        let src = Keywords::ALL
            .iter()
            .map(|kw| kw.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        let mut parser = CarbideLexer::from_src(&src);
        let tokens = parser.lex().expect("Parsing should succeed");

        let mut expected = Vec::new();
        let mut start = 0usize;

        for kw in Keywords::ALL {
            let lit = kw.as_str();
            let end = start + lit.len();
            expected.push(Token::new(
                Tokens::Keyword(*kw),
                start as u64..end as u64,
                lit,
            ));
            start = end + 1;
        }

        assert_eq!(tokens, expected);
    }
}
