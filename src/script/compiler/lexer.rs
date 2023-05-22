use std::alloc::{alloc, dealloc, Layout};
use std::iter::Peekable;
use std::str::Chars;
use std::fmt::Display;
use std::collections::VecDeque;
use phf::{Map, phf_map};
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Identifier(i) => i.clone(),
            Token::Keyword(word) => word.to_string(),
            Token::Operator(op) => op.to_string(),
            Token::OperatorAssign(op) => op.to_string() + "=",
            Token::Literal(lit) => lit.to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LSquare => "[".to_string(),
            Token::RSquare => "]".to_string(),
            Token::LCurly => "{".to_string(),
            Token::RCurly => "}".to_string(),
            Token::Comma => ",".to_string(),
            Token::Dot => ".".to_string(),
            Token::Colon => ":".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Arrow => "->".to_string(),
            Token::ThickArrow => "=>".to_string(),
            Token::Eof => "eof".to_string()
        };
        f.write_str(&s)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Keyword {
    Include,
    Use,
    Args,
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Int,
    Float,
    String,
    Bool,
    Char
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Keyword::Include => "include",
            Keyword::Use => "use",
            Keyword::Args => "args",
            Keyword::Let => "let",
            Keyword::Const => "const",
            Keyword::Fn => "fn",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::In => "in",
            Keyword::Return => "return",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Int => "int",
            Keyword::Float => "float",
            Keyword::String => "String",
            Keyword::Bool => "bool",
            Keyword::Char => "char"
        };
        f.write_str(s)
    }
}

static KEYWORDS: Map<&'static str, Keyword> = phf_map! {
    "include" => Keyword::Include,
    "use" => Keyword::Use,
    "args" => Keyword::Args,
    "let" => Keyword::Let,
    "const" => Keyword::Const,
    "fn" => Keyword::Fn,
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "while" => Keyword::While,
    "for" => Keyword::For,
    "in" => Keyword::In,
    "return" => Keyword::Return,
    "break" => Keyword::Break,
    "continue" => Keyword::Continue,
    "int" => Keyword::Int,
    "float" => Keyword::Float,
    "String" => Keyword::String,
    "bool" => Keyword::Bool,
    "char" => Keyword::Char
};

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
    ArithmeticRightShift,
    Range
}

impl Operator {
    pub fn is_unary(&self) -> bool {
        match self {
            Operator::PlusPlus |
            Operator::MinusMinus |
            Operator::Not => true,
            _ => false
        }
    }

    pub fn is_pre_unary(&self) -> bool {
        match self {
            Operator::PlusPlus |
            Operator::MinusMinus |
            Operator::Not |
            Operator::Minus => true,
            _ => false
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Plus => "+",
            Operator::PlusPlus => "++",
            Operator::Minus => "-",
            Operator::MinusMinus => "--",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",
            Operator::Assign => "=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::LessOrEqual => "<=",
            Operator::GreaterOrEqual => ">=",
            Operator::And => "&&",
            Operator::BitwiseAnd => "&",
            Operator::Or => "||",
            Operator::BitwiseOr => "|",
            Operator::Xor => "^",
            Operator::Not => "!",
            Operator::LeftShift => "<<",
            Operator::LogicalRightShift => ">>",
            Operator::ArithmeticRightShift => ">>>",
            Operator::Range => ".."
        };
        f.write_str(s)
    }
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

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::Char(c) => c.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Null => "null".to_string()
        };
        f.write_str(&s)
    }
}

pub fn err(msg: String) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub struct Lexer {
    ptr: *mut String,
    chars: Peekable<Chars<'static>>,
    reverted: VecDeque<Token>
}

impl Lexer {
    pub fn new(src: String) -> Self {
        unsafe {
            let ptr = alloc(Layout::new::<String>()) as *mut String;
            ptr.write(src);
            Lexer {
                chars: ptr.as_ref().unwrap().chars().peekable(),
                ptr,
                reverted: VecDeque::new()
            }
        }
    }

    pub fn revert(&mut self, token: Token) {
        self.reverted.push_back(token);
    }

    pub fn next_token(&mut self) -> Token {
        if !self.reverted.is_empty() {
            return self.reverted.pop_front().unwrap();
        }
        while let Some(ch) = self.chars.next() {
            match ch {
                ch if ch.is_whitespace() => {}
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
                    let mut buffer = String::new();
                    buffer.push(ch);
                    while let Some(ch) = self.chars.peek() {
                        if ch.is_ascii_alphanumeric() || *ch == '_' {
                            buffer.push(self.chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    return if buffer == "true" {
                        Token::Literal(Literal::Bool(true))
                    }
                    else if buffer == "false" {
                        Token::Literal(Literal::Bool(false))
                    }
                    else if buffer == "null" {
                        Token::Literal(Literal::Null)
                    }
                    else if KEYWORDS.contains_key(&buffer) {
                        Token::Keyword(KEYWORDS[&buffer].clone())
                    }
                    else {
                        Token::Identifier(buffer)
                    };
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
                        Token::Literal(Literal::Float(buffer.parse().unwrap_or_else(|e| {
                            err(format!("Failed to parse float \"{}\": {}", buffer, e));
                            0f64
                        })))
                    }
                    else {
                        Token::Literal(Literal::Integer(buffer.parse().unwrap_or_else(|e| {
                            err(format!("Failed to parse integer \"{}\": {}", buffer, e));
                            0i64
                        })))
                    }
                }
                ch => {
                    return match ch {
                        '(' => Token::LParen,
                        ')' => Token::RParen,
                        '[' => Token::LSquare,
                        ']' => Token::RSquare,
                        '{' => Token::LCurly,
                        '}' => Token::RCurly,
                        ',' => Token::Comma,
                        '.' => {
                            match self.chars.peek() {
                                Some('.') => {
                                    self.chars.next();
                                    Token::Operator(Operator::Range)
                                }
                                _ => Token::Dot
                            }
                        }
                        ':' => Token::Colon,
                        ';' => Token::Semicolon,
                        '-' => {
                            match self.chars.peek() {
                                Some('>') => {
                                    self.chars.next();
                                    Token::Arrow
                                }
                                Some('-') => {
                                    self.chars.next();
                                    Token::Operator(Operator::MinusMinus)
                                }
                                _ => Token::Operator(Operator::Minus)
                            }
                        }
                        '=' => {
                            match self.chars.peek() {
                                Some('>') => {
                                    self.chars.next();
                                    Token::ThickArrow
                                }
                                Some('=') => {
                                    self.chars.next();
                                    Token::Operator(Operator::Equal)
                                }
                                _ => Token::Operator(Operator::Assign)
                            }
                        }
                        '+' => {
                            match self.chars.peek() {
                                Some('+') => {
                                    self.chars.next();
                                    Token::Operator(Operator::PlusPlus)
                                }
                                Some('=') => {
                                    self.chars.next();
                                    Token::OperatorAssign(Operator::Plus)
                                }
                                _ => Token::Operator(Operator::Plus)
                            }
                        }
                        '*' => {
                            if let Some('=') = self.chars.peek() {
                                self.chars.next();
                                Token::OperatorAssign(Operator::Multiply)
                            }
                            else {
                                Token::Operator(Operator::Multiply)
                            }
                        }
                        '%' => {
                            if let Some('=') = self.chars.peek() {
                                self.chars.next();
                                Token::OperatorAssign(Operator::Modulo)
                            }
                            else {
                                Token::Operator(Operator::Modulo)
                            }
                        }
                        '!' => {
                            if let Some('=') = self.chars.peek() {
                                self.chars.next();
                                Token::Operator(Operator::NotEqual)
                            }
                            else {
                                Token::Operator(Operator::Not)
                            }
                        }
                        '<' => {
                            match self.chars.peek() {
                                Some('=') => {
                                    self.chars.next();
                                    Token::OperatorAssign(Operator::LessOrEqual)
                                }
                                Some('<') => {
                                    self.chars.next();
                                    if let Some('=') = self.chars.peek() {
                                        self.chars.next();
                                        Token::OperatorAssign(Operator::LeftShift)
                                    }
                                    else {
                                        Token::Operator(Operator::LeftShift)
                                    }
                                }
                                _ => Token::Operator(Operator::LessThan)
                            }
                        }
                        '>' => {
                            match self.chars.peek() {
                                Some('=') => {
                                    self.chars.next();
                                    Token::OperatorAssign(Operator::GreaterOrEqual)
                                }
                                Some('>') => {
                                    self.chars.next();
                                    match self.chars.peek() {
                                        Some('=') => {
                                            self.chars.next();
                                            Token::Operator(Operator::GreaterOrEqual)
                                        }
                                        Some('>') => {
                                            self.chars.next();
                                            if let Some('=') = self.chars.peek() {
                                                self.chars.next();
                                                Token::OperatorAssign(Operator::LogicalRightShift)
                                            }
                                            else {
                                                Token::Operator(Operator::LogicalRightShift)
                                            }
                                        }
                                        _ => Token::Operator(Operator::ArithmeticRightShift)
                                    }
                                }
                                _ => Token::Operator(Operator::GreaterThan)
                            }
                        }
                        '&' => {
                            match self.chars.peek() {
                                Some('&') => {
                                    self.chars.next();
                                    Token::Operator(Operator::And)
                                }
                                Some('=') => {
                                    self.chars.next();
                                    Token::OperatorAssign(Operator::BitwiseAnd)
                                }
                                _ => Token::Operator(Operator::BitwiseAnd)
                            }
                        }
                        '|' => {
                            match self.chars.peek() {
                                Some('|') => {
                                    self.chars.next();
                                    Token::Operator(Operator::Or)
                                }
                                Some('=') => {
                                    self.chars.next();
                                    Token::OperatorAssign(Operator::BitwiseOr)
                                }
                                _ => Token::Operator(Operator::BitwiseOr)
                            }
                        }
                        '^' => {
                            if let Some('=') = self.chars.peek() {
                                self.chars.next();
                                Token::OperatorAssign(Operator::Xor)
                            }
                            else {
                                Token::Operator(Operator::Xor)
                            }
                        }
                        _ => {
                            err(format!("Illegal character: '{}'", ch));
                            Token::Literal(Literal::Char(ch))
                        }
                    }
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