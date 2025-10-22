use carbide_lexer::operators::{BinaryOperators, UnaryOperators};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Basic types like `int`, `float`, `string`, `bool`
    Named(String),
    /// Function type: (`param_types`) -> `return_type`
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    /// Array type: [`element_type`]
    Array(Box<Type>),
    /// Unit type
    Unit,
}

impl Type {
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }
}

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
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Variable declaration, like `let name: type = value;`
    LetDeclaration {
        name: String,
        type_annotation: Option<Type>,
        initializer: Option<Expression>,
    },

    /// Function declaration
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
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
