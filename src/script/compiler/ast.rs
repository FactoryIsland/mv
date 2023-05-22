use crate::script::compiler::lexer::{Literal, Operator};

#[derive(Debug, Clone)]
pub struct Program {
    elements: Vec<Element>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }
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
    Block(Block),
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
pub enum Type {
    Integer,
    Float,
    Char,
    String,
    Bool
}

#[derive(Debug, Clone)]
pub struct IncludeStatement {
    pub what: String,
}

#[derive(Debug, Clone)]
pub struct UseStatement {
    pub what: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub ty: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: String,
    pub arguments: Vec<Expression>,
}