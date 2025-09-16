use logos::Logos;

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tokens {
    // Primitives
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),
    #[regex(r#""([^"\\]|\\[nrt"\\])*""#, |lex| {
        let slice = lex.slice();

        let content = &slice[1..slice.len()-1];
        Some(content.replace("\\n", "\n")
                   .replace("\\r", "\r")
                   .replace("\\t", "\t")
                   .replace("\\\"", "\"")
                   .replace("\\\\", "\\"))
    })]
    String(String),
    #[regex(r"true|false", |lex| match lex.slice() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })]
    Boolean(bool),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    // Delimiters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("~")]
    Tilde,
    #[token(",")]
    Comma,

    // Boolean
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Bang,
    #[token("=")]
    Equals,
    #[token("==")]
    EqualsEquals,
    #[token("!=")]
    NotEquals,

    // Arithmetic
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("&")]
    Ampersand,

    // Keywords
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,

    #[token("loop")]
    Loop,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Comparison operators
    #[token("<")]
    Less,
    #[token("<=")]
    LessOrEq,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterOrEq,

    // Unary operators
    #[token("--")]
    MinusMinus,
    #[token("++")]
    PlusPlus,

    NUL,
}

impl ToString for Tokens {
    fn to_string(&self) -> String {
        match self {
            Tokens::Integer(n) => format!("<int {n}>"),
            Tokens::Float(n) => format!("<float {n}>"),
            Tokens::String(s) => format!("<string {s}>"),
            Tokens::Boolean(b) => format!("<bool {b}>"),
            _ => format!("{:?}", self)
        }.to_string()
    }
}