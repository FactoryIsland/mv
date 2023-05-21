use std::alloc::{alloc, dealloc, Layout};
use std::iter::Peekable;
use std::str::Chars;
use crate::script::utils::parse_char;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    OperatorAssign(Operator),
    Literal(Literal),
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    ThickArrow,
    Eof
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Keyword {
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Return
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operator {
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    And,
    BitwiseAnd,
    Or,
    BitwiseOr,
    Xor,
    Not,
    LeftShift,
    LogicalRightShift,
    ArithmeticRightShift
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Bool(bool),
    Null
}

pub fn err(msg: String) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub struct Lexer {
    ptr: *mut String,
    chars: Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        unsafe {
            let ptr = alloc(Layout::new::<String>()) as *mut String;
            ptr.write(src);
            Lexer {
                chars: ptr.as_ref().unwrap().chars().peekable(),
                ptr,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.chars.next() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => {}
                '/' => {
                    match self.chars.peek() {
                        Some('/') => {
                            while let Some(ch) = self.chars.next() {
                                if ch == '\n' {
                                    break;
                                }
                            }
                        }
                        Some('*') => {
                            while let Some(ch) = self.chars.next() {
                                if ch == '*' {
                                    if let Some('/') = self.chars.peek() {
                                        self.chars.next();
                                        break;
                                    }
                                }
                            }
                        }
                        Some('=') => {
                            self.chars.next();
                            return Token::OperatorAssign(Operator::Divide);
                        }
                        _ => {
                            return Token::Operator(Operator::Divide);
                        }
                    }
                }
                '\'' => {
                    let mut buffer = String::new();
                    while let Some(ch) = self.chars.next() {
                        if ch == '\'' {
                            break;
                        }
                        buffer.push(ch);
                    }
                    return Token::Literal(Literal::Char(parse_char(&buffer, err)));
                }
                '"' => {
                    let mut buffer = String::new();
                    while let Some(ch) = self.chars.next() {
                        if ch == '"' {
                            break;
                        }
                        buffer.push(ch);
                    }
                    return Token::Literal(Literal::String(buffer));
                }
                ch if ch.is_alphabetic() => {

                }
                ch if ch.is_ascii_digit() => {
                    let mut buffer = String::new();
                    buffer.push(ch);
                    while let Some(ch) = self.chars.peek() {
                        if ch.is_ascii_digit() || *ch == '.' {
                            buffer.push(*ch);
                            self.chars.next();
                        } else {
                            if *ch == 'f' {
                                self.chars.next();
                                if !buffer.contains('.') {
                                    buffer.push('.');
                                }
                            }
                            break;
                        }
                    }
                    return if buffer.contains('.') {
                        Token::Literal(Literal::Float(buffer.parse().unwrap()))
                    }
                    else {
                        Token::Literal(Literal::Integer(buffer.parse().unwrap()))
                    }
                }
                ch => {

                }
            }
        }
        Token::Eof
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, Layout::new::<String>());
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::Eof {
            None
        } else {
            Some(token)
        }
    }
}