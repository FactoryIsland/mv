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
                Keyword::Include => {},
                Keyword::Use => {},
                Keyword::Const => {},
                Keyword::Let => {},
                Keyword::Fn => {},
                _ => Err(format!("Unexpected token, expected 'include' | 'use' | 'const' | 'let' | 'fn', found {:?}", keyword).into())
            }
        }
        else {
            Err(format!("Unexpected token, expected Keyword, found {:?}", token).into())
        }
    }
}