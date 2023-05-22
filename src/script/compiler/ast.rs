use crate::script::compiler::lexer::*;
use crate::script::compiler::parser::*;

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
    Empty,
}

#[derive(Debug, Clone)]
pub enum TopLevelStatement {
    Declaration(Declaration),
    Include(String),
    Use(Vec<String>),
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

impl Expression {
    pub fn infer_type(&self) -> Option<Type> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Char,
    String,
    Bool
}

impl TryFrom<Keyword> for Type {
    type Error = ParseError;

    fn try_from(k: Keyword) -> Result<Self, ParseError> {
        match k {
            Keyword::Int => Ok(Type::Int),
            Keyword::Float => Ok(Type::Float),
            Keyword::String => Ok(Type::String),
            Keyword::Bool => Ok(Type::Bool),
            Keyword::Char => Ok(Type::Char),
            _ => Err(format!("Type: Invalid keyword for type {}", k).into())
        }
    }
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