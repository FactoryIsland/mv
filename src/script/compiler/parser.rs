use super::lexer::*;
use super::ast::*;
use std::error::Error;
use std::fmt::Display;

pub struct Parser {
    lexer: Lexer,
    program: Program,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        ParseError {
            message: s.to_string(),
        }
    }
}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError {
            message: s,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParseError {}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            program: Program::new(),
        }
    }

    pub fn parse(mut self) -> Result<Program, ParseError> {
        while let Some(token) = self.lexer.next() {
            let element = self.parse_element(token)?;
            self.program.push(element);
        }
        Ok(self.program)
    }

    fn parse_element(&mut self, token: Token) -> Result<Element, ParseError> {
        if let Token::Keyword(keyword) = token {
            match keyword {
                Keyword::Include => {
                    let token = self.lexer.next_token();
                    return if let Token::Identifier(name) = token {
                        let token = self.lexer.next_token();
                        if let Token::Semicolon = token {
                            Ok(Element::Statement(TopLevelStatement::Include(name)))
                        }
                        else {
                            Err(format!("Include: Unexpected token, expected ';', got {}", token).into())
                        }
                    }
                    else {
                        Err(format!("Include: Unexpected token, expected identifier, got {}", token).into())
                    };
                },
                Keyword::Use => Ok(Element::Statement(TopLevelStatement::Use(self.parse_use()?))),
                Keyword::Const => {
                    let declaration = self.parse_declaration(true)?;
                    if declaration.value.is_none() {
                        Err("Const: Must have a value".into())
                    }
                    else {
                        //do stuff later
                        Ok(Element::Empty)
                    }
                },
                Keyword::Let => Ok(Element::Statement(TopLevelStatement::Declaration(self.parse_declaration(true)?))),
                Keyword::Fn => Ok(Element::Function(self.parse_fn()?)),
                _ => Err(format!("File: Unexpected keyword, expected 'include' | 'use' | 'const' | 'let' | 'fn', found {}", keyword).into())
            }
        }
        else {
            Err(format!("File: Unexpected token, expected Keyword, found {}", token).into())
        }
    }

    fn parse_use(&mut self) -> Result<Vec<String>, ParseError> {
        let mut res = Vec::new();
        let token = self.lexer.next_token();
        if let Token::Identifier(usage) = token {
            res.push(usage);
        }
        else {
            return Err(format!("Use: Unexpected token, expected Identifier, found {}", token).into());
        }
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Semicolon => break,
                Token::Comma => {
                    let token = self.lexer.next_token();
                    if let Token::Identifier(usage) = token {
                        res.push(usage);
                    }
                    else {
                        return Err(format!("Use: Unexpected token, expected Identifier, found {}", token).into());
                    }
                },
                _ => {
                    return Err(format!("Use: Unexpected token, expected ';' or ',', found {}", token).into())
                }
            }
        }
        Ok(res)
    }

    fn parse_declaration(&mut self, semi: bool) -> Result<Declaration, ParseError> {
        let token = self.lexer.next_token();
        if let Token::Identifier(name) = token {
            let mut ty = None;
            let mut token = self.lexer.next_token();
            if let Token::Colon = token {
                token = self.lexer.next_token();
                if let Token::Keyword(word) = token {
                    ty = Some(Type::try_from(word)?);
                }
                else {
                    return Err(format!("Let/Const: Unexpected token, expected Type, found {}", token).into());
                }
                token = self.lexer.next_token();
            }
            match token {
                Token::Operator(Operator::Assign) => {
                    let value = self.parse_expression()?;
                    if semi {
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Let/Const: Unexpected token, expected ';', found {}", token).into());
                        }
                    }
                    if let Some(ty) = ty {
                        Ok(Declaration {
                            name,
                            ty,
                            value: Some(value),
                        })
                    }
                    else {
                        let ty = value.infer_type();
                        if let Some(ty) = ty {
                            Ok(Declaration {
                                name,
                                ty,
                                value: Some(value),
                            })
                        }
                        else {
                            Err(format!("Let/Const: Cannot infer type for variable {}, please add a type annotation", name).into())
                        }
                    }
                }
                Token::Semicolon => {
                    if let Some(ty) = ty {
                        Ok(Declaration {
                            name,
                            ty,
                            value: None
                        })
                    }
                    else {
                        Err("Let/Const: Variable without initial value must be given a type annotation".into())
                    }
                }
                _ => Err(format!("Let/Const: Unexpected token, expected '=' or ';', found {}", token).into())
            }
        }
        else {
            Err(format!("Let/Const: Unexpected token, expected Identifier, found {}", token).into())
        }
    }

    fn parse_fn(&mut self) -> Result<Function, ParseError> {
        let token = self.lexer.next_token();
        if let Token::Identifier(name) = token {
            let token = self.lexer.next_token();
            if token != Token::LParen {
                return Err(format!("Fn: Unexpected token, expected '(', found {}", token).into());
            }
            let mut parameters = Vec::new();
            let mut token = self.lexer.next_token();
            while token != Token::RParen {
                if token == Token::Comma {
                    token = self.lexer.next_token();
                }
                if let Token::Identifier(name) = token {
                    let token = self.lexer.next_token();
                    if token != Token::Colon {
                        return Err(format!("Fn: Unexpected token, expected ':', found {}", token).into());
                    }
                    let token = self.lexer.next_token();
                    if let Token::Keyword(word) = token {
                        let ty = Type::try_from(word)?;
                        parameters.push((name, ty));
                    }
                    else {
                        return Err(format!("Let/Const: Unexpected token, expected Type, found {}", token).into());
                    }
                }
                else {
                    return Err(format!("Fn: Unexpected token, expected Identifier, found {}", token).into());
                }
                token = self.lexer.next_token();
            }
            let mut token = self.lexer.next_token();
            let mut ty = Type::Void;
            if let Token::Arrow = token {
                token = self.lexer.next_token();
                if let Token::Keyword(word) = token {
                    ty = Type::try_from(word)?;
                }
                else if let Token::LParen = token {
                    token = self.lexer.next_token();
                    if token != Token::RParen {
                        return Err(format!("Fn: Unexpected token, tuples are not supported, expected ')', found {}", token).into());
                    }
                }
                else {
                    return Err(format!("Fn: Unexpected token, expected Type, found {}", token).into());
                }
                token = self.lexer.next_token();
            }
            if token != Token::LCurly {
                return Err(format!("Fn: Unexpected token, expected '{{', found {}", token).into());
            }
            let mut body = Vec::new();
            token = self.lexer.next_token();
            while token != Token::RCurly {
                self.lexer.revert(token);
                body.push(self.parse_statement(true)?);
                token = self.lexer.next_token();
            }
            Ok(Function {
                name,
                parameters,
                return_type: ty,
                body: Block {
                    statements: body
                },
            })
        }
        else {
            return Err(format!("Fn: Unexpected token, expected Identifier, found {}", token).into());
        }
    }

    fn parse_statement(&mut self, semi: bool) -> Result<Statement, ParseError> {
        let token = self.lexer.next_token();
        match token {
            Token::Keyword(word) => {
                match word {
                    Keyword::Let => Ok(Statement::Declaration(self.parse_declaration(semi)?)),
                    Keyword::If => {
                        if !semi {
                            return Err("If cannot be used inside a for initialization or next component!".into());
                        }
                        let condition = self.parse_expression()?;
                        let body = Box::new(self.parse_statement(true)?);
                        let token = self.lexer.next_token();
                        if token == Token::Keyword(Keyword::Else) {
                            let else_body = Box::new(self.parse_statement(true)?);
                            Ok(Statement::If(IfStatement {
                                condition,
                                body,
                                else_body: Some(else_body),
                            }))
                        }
                        else {
                            self.lexer.revert(token);
                            Ok(Statement::If(IfStatement {
                                condition,
                                body,
                                else_body: None,
                            }))
                        }
                    }
                    Keyword::While => {
                        if !semi {
                            return Err("While cannot be used inside a for initialization or next component!".into());
                        }
                        let condition = self.parse_expression()?;
                        let body = Box::new(self.parse_statement(true)?);
                        Ok(Statement::While(WhileStatement {
                            condition,
                            body,
                        }))
                    }
                    Keyword::For => {
                        if !semi {
                            return Err("For cannot be used inside a for initialization or next component!".into());
                        }
                        let init = self.parse_statement(true)?;
                        let condition = self.parse_expression()?;
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("For: Unexpected token, expected ';', found {}", token).into());
                        }
                        let next = self.parse_statement(false)?;
                        let mut body = self.parse_statement(true)?;
                        Ok(Statement::For(ForStatement {
                            init: Box::new(init),
                            condition,
                            next: Box::new(next),
                            body: Box::new(body),
                        }))
                    }
                    Keyword::Break => {
                        if !semi {
                            return Err("Break cannot be used inside a for initialization or next component!".into());
                        }
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Break: Unexpected token, expected ';', found {}", token).into());
                        }
                        Ok(Statement::Break)
                    }
                    Keyword::Continue => {
                        if !semi {
                            return Err("Continue cannot be used inside a for initialization or next component!".into());
                        }
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Continue: Unexpected token, expected ';', found {}", token).into());
                        }
                        Ok(Statement::Continue)
                    }
                    Keyword::Return => {
                        if !semi {
                            return Err("Return cannot be used inside a for initialization or next component!".into());
                        }
                        let token = self.lexer.next_token();
                        if let Token::Semicolon = token {
                            Ok(Statement::Return(None))
                        }
                        else {
                            self.lexer.revert(token);
                            let value = self.parse_expression()?;
                            let token = self.lexer.next_token();
                            if token != Token::Semicolon {
                                return Err(format!("Return: Unexpected token, expected ';', found {}", token).into());
                            }
                            Ok(Statement::Return(Some(value)))
                        }
                    }
                    _ => {
                        return Err(format!("Statement: Unexpected keyword, found {}", word).into());
                    }
                }
            }
            Token::LCurly => {
                let mut body = Vec::new();
                let mut token = self.lexer.next_token();
                while token != Token::RCurly {
                    self.lexer.revert(token);
                    body.push(self.parse_statement(true)?);
                    token = self.lexer.next_token();
                }
                Ok(Statement::Block(Block {
                    statements: body
                }))
            }
            Token::Identifier(name) => {
                let next = self.lexer.next_token();
                if let Token::OperatorAssign(operator) = next {
                    let extra = self.parse_expression()?;
                    if semi {
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Assignment: Unexpected token, expected ';', found {}", token).into());
                        }
                    }
                    let left = Box::new(Expression::Identifier(name.clone()));
                    Ok(Statement::Assignment(Assignment {
                        name,
                        value: Expression::Binary(BinaryExpression {
                            left,
                            operator,
                            right: Box::new(extra)
                        })
                    }))
                }
                else if let Token::Operator(Operator::Assign) = next {
                    let value = self.parse_expression()?;
                    if semi {
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Assignment: Unexpected token, expected ';', found {}", token).into());
                        }
                    }
                    Ok(Statement::Assignment(Assignment {
                        name,
                        value
                    }))
                }
                else {
                    self.lexer.revert(Token::Identifier(name));
                    self.lexer.revert(next);
                    let expr = self.parse_expression()?;
                    if semi {
                        let token = self.lexer.next_token();
                        if token != Token::Semicolon {
                            return Err(format!("Expression: Unexpected token, expected ';', found {}", token).into());
                        }
                    }
                    Ok(Statement::Expression(expr))
                }
            }
            Token::Semicolon => Ok(Statement::Noop),
            _ => {
                self.lexer.revert(token);
                let expr = self.parse_expression()?;
                if semi {
                    let token = self.lexer.next_token();
                    if token != Token::Semicolon {
                        return Err(format!("Expression: Unexpected token, expected ';', found {}", token).into());
                    }
                }
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_expression_with_precedence(0)
    }

    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
        let mut lhs = self.parse_primary_expression()?;
        let mut token = self.lexer.next_token();
        while let Token::Operator(op) = token {
            let precedence = op.precedence()?;

            if precedence < min_precedence {
                token = Token::Operator(op);
                break;
            }

            let mut rhs = self.parse_primary_expression()?;

            let mut inner_token = self.lexer.next_token();
            while let Token::Operator(inner_op) = inner_token {
                let inner_precedence = inner_op.precedence()?;

                if inner_precedence < precedence {
                    self.lexer.revert(Token::Operator(inner_op));
                    inner_token = self.lexer.next_token();
                    break;
                }

                let extra = self.parse_expression_with_precedence(inner_precedence)?;
                rhs = Expression::Binary(BinaryExpression {
                    left: Box::new(rhs),
                    operator: inner_op,
                    right: Box::new(extra)
                });
                inner_token = self.lexer.next_token();
            }
            self.lexer.revert(inner_token);
            lhs = Expression::Binary(BinaryExpression {
                left: Box::new(lhs),
                operator: op,
                right: Box::new(rhs)
            });
            token = self.lexer.next_token();
        }
        self.lexer.revert(token);

        Ok(lhs)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.lexer.next_token();
        match token {
            Token::Keyword(word) if word == Keyword::Args => {
                let token = self.lexer.next_token();
                if token != Token::LSquare {
                    return Err(format!("Args: Unexpected token, expected '[', found {}", token).into());
                }
                let expr = self.parse_expression()?;
                let token = self.lexer.next_token();
                if token!= Token::RSquare {
                    return Err(format!("Args: Unexpected token, expected ']', found {}", token).into());
                }
                Ok(Expression::Argument(Box::new(expr)))
            }
            Token::Operator(op) if op.is_unary() => {
                let operand = self.parse_expression()?;
                Ok(Expression::Unary(UnaryExpression {
                    operator: op,
                    expr: Box::new(operand)
                }))
            }
            Token::Identifier(name) => {
                let token = self.lexer.next_token();
                match token {
                    Token::LParen => {
                        let arguments = self.parse_arguments()?;
                        Ok(Expression::Call(CallExpression {
                            function: name,
                            arguments,
                        }))
                    }
                    _ => {
                        self.lexer.revert(token);
                        Ok(Expression::Identifier(name))
                    }
                }
            }
            Token::LParen => {
                let expr = self.parse_expression()?;
                let token = self.lexer.next_token();
                if token != Token::RParen {
                    return Err(format!("Expression: Unexpected token, expected ')', found {}", token).into());
                }
                Ok(expr)
            }
            Token::Literal(literal) => Ok(Expression::Literal(literal)),
            _ => Err(format!("Expression: Unexpected token, expected Identifier, Literal, UnaryOperator, args or '(', found {}", token).into()),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut arguments = Vec::new();
        let mut token = self.lexer.next_token();
        loop {
            match token {
                Token::Comma => {}
                Token::RParen => return Ok(arguments),
                _ => {
                    self.lexer.revert(token);
                    let expr = self.parse_expression()?;
                    arguments.push(expr);
                }
            }
            token = self.lexer.next_token();
        }
    }
}