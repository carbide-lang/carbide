use carbide_lexer::operators::{BinaryOperators, UnaryOperators};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Literal value
    Literal(LiteralValue),

    /// Identifier reference
    Identifier(String),

    /// Binary operation: left op right
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperators,
        right: Box<Expression>,
    },

    /// Unary operation: op operand
    UnaryOp {
        operator: UnaryOperators,
        operand: Box<Expression>,
    },

    /// Assignment: target = value
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Function call: callee(args)
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    /// Array/object indexing: target[index]
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },

    /// Member access: target.member
    MemberAccess {
        target: Box<Expression>,
        member: String,
    },

    /// Grouped expression: (expr)
    Grouped(Box<Expression>),

    /// Array literal: [expr, expr, ...]
    Array(Vec<Expression>),

    /// Interpolated string with expressions
    InterpolatedString { parts: Vec<StringPart> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Variable declaration, like `let name = value;`
    LetDeclaration {
        name: String,
        initializer: Option<Expression>,
    },

    /// Function declaration
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },

    /// Return statement, like `return expr;`
    Return(Option<Expression>),

    /// If statement
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    /// While loop
    While {
        condition: Expression,
        body: Vec<Statement>,
    },

    /// For loop
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Vec<Statement>,
    },

    /// Block of [`Statements`][`Statement`]
    Block(Vec<Statement>),

    /// [`Expression`] statement
    Expression(Expression),

    /// `break` statement
    Break,

    /// `continue` statement
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    #[must_use]
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}
