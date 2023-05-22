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

    pub fn parse_element(&mut self, token: Token) -> Result<Element, ParseError> {
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
                    let token = self.lexer.next_token();
                    let name = if let Token::Identifier(n) = token {
                        n
                    }
                    else {
                        return Err(format!("Const: Unexpected token, expected identifier, got {}", token).into());
                    };

                    Ok(Element::Empty)
                },
                Keyword::Let => Ok(Element::Statement(TopLevelStatement::Declaration(self.parse_declaration()?))),
                Keyword::Fn => Ok(Element::Function(self.parse_fn()?)),
                _ => Err(format!("File: Unexpected keyword, expected 'include' | 'use' | 'const' | 'let' | 'fn', found {}", keyword).into())
            }
        }
        else {
            Err(format!("File: Unexpected token, expected Keyword, found {}", token).into())
        }
    }

    pub fn parse_use(&mut self) -> Result<Vec<String>, ParseError> {
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
                Token::Comma => {},
                _ => {
                    return Err(format!("Use: Unexpected token, expected ';' or ',', found {}", token).into())
                }
            }
        }
        let token = self.lexer.next_token();
        if let Token::Identifier(usage) = token {
            res.push(usage);
        }
        else {
            return Err(format!("Use: Unexpected token, expected Identifier, found {}", token).into());
        }
        Ok(res)
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        Err("".into())
    }

    pub fn parse_fn(&mut self) -> Result<Function, ParseError> {
        Err("".into())
    }
}