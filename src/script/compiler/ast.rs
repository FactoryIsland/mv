use crate::script::compiler::lexer::*;
use crate::script::compiler::parser::*;

#[derive(Debug, Clone)]
pub struct Program {
    pub elements: Vec<Element>,
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
    Break,
    Continue,
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(Option<Expression>),
    Noop,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
    Argument(Box<Expression>)
}

impl Expression {
    pub fn infer_type(&self) -> Option<Type> {
        match self {
            Expression::Literal(literal) => {
                match literal {
                    Literal::Integer(_) => Some(Type::Int),
                    Literal::Float(_) => Some(Type::Float),
                    Literal::Char(_) => Some(Type::Char),
                    Literal::String(_) => Some(Type::String),
                    Literal::Bool(_) => Some(Type::Bool),
                    Literal::Null => None
                }
            }
            Expression::Binary(binary) => {
                let a = binary.left.infer_type();
                let b = binary.right.infer_type();
                if a.is_none() {
                    b
                }
                else if b.is_none() {
                    a
                }
                else {
                    let a = a.unwrap();
                    let b = b.unwrap();
                    if a == b {
                        Some(a)
                    }
                    else if a == Type::String || b == Type::String {
                        Some(Type::String)
                    }
                    else if a == Type::Bool || b == Type::Bool {
                        Some(Type::Bool)
                    }
                    else if a == Type::Float || b == Type::Float {
                        Some(Type::Float)
                    }
                    else {
                        Some(a)
                    }
                }
            }
            Expression::Unary(unary) => {
                unary.expr.infer_type()
            }
            Expression::Argument(_) => Some(Type::String),
            _ => None
        }
    }

    pub fn is_null(&self) -> bool {
        if let Expression::Literal(Literal::Null) = self {
            true
        }
        else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        if let Expression::Literal(Literal::Integer(i)) = self {
            *i == 0
        }
        else if let Expression::Literal(Literal::Float(f)) = self {
            *f == 0.0
        }
        else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Char,
    String,
    Bool,
    Void
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
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
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
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub expr: Box<Expression>,
    pub operator: Operator
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: String,
    pub arguments: Vec<Expression>,
}