use crate::script::compiler::lexer::{Literal, Operator};

#[derive(Debug, Clone)]
pub struct Program {
    elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Statement(TopLevelStatement),
    Function(Function),
}

#[derive(Debug, Clone)]
pub enum TopLevelStatement {
    Declaration(Declaration),
    Include(IncludeStatement),
    Use(UseStatement),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Declaration(Declaration),
    Assignment(Assignment),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement)
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(BinaryExpression),
    Call(CallExpression)
}

#[derive(Debug, Clone)]
pub struct IncludeStatement {
    what: String,
}

#[derive(Debug, Clone)]
pub struct UseStatement {
    what: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    name: String,
    ty: Type,
    value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    name: String,
    value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    condition: Expression,
    then_branch: Box<Statement>,
    else_branch: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    condition: Expression,
    body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    variable: String,
    iterable: Expression,
    body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    parameters: Vec<(String, Type)>,
    return_type: Option<Type>,
    body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    function: String,
    arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer,
    Float,
    Char,
    String,
    Bool
}