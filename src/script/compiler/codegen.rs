use crate::script::compiler::ast::{Element, Expression, ForStatement, Function, IfStatement, Program, Statement, TopLevelStatement, WhileStatement};
use crate::script::compiler::lexer::{Literal, Operator};

pub struct Generator {
    pub program: Program,
}

impl Generator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn generate(self) -> String {
        let mut data = StaticData {
            preload_name: "static".to_string(),
            preload_code: String::new(),
            next_label: String::new(),
            label_stack: Vec::new()
        };

        for e in &self.program.elements {
            if let Element::Function(f) = e {
                if f.name == data.preload_name {
                    data.preload_name = data.preload_name + "0";
                }
            }
        }

        let code = self.program.codegen(&mut data);

        if data.preload_code.is_empty() {
            format!(".named\n{}@{}:\nret", code, data.preload_name)
        }
        else {
            format!(".named\n{}@{}:\n{}\nret", code, data.preload_name, data.preload_code)
        }
    }
}

pub struct StaticData {
    pub preload_name: String,
    pub preload_code: String,
    pub next_label: String,
    pub label_stack: Vec<String>,
}

impl StaticData {
    pub fn next_label(&mut self) -> String {
        if self.next_label.is_empty() {
            self.next_label = "L0".to_string();
        }
        else {
            let label = self.next_label.split_at(1).1;
            self.next_label = format!("L{}", label.parse::<u32>().unwrap() + 1);
        }
        self.next_label.clone()
    }
}

pub trait Codegen: Sized {
    fn codegen(self, data: &mut StaticData) -> String;
    fn codegen_conditional(self, data: &mut StaticData, true_label: &str, false_label: &str) -> String {
        panic!("This element doesn't support conditional code generation");
    }
}

impl Codegen for Program {
    fn codegen(self, data: &mut StaticData) -> String {
        let mut code = String::new();
        for stmt in self.elements {
            code.push_str(&stmt.codegen(data));
        }
        code
    }
}

impl Codegen for Element {
    fn codegen(self, data: &mut StaticData) -> String {
        match self {
            Element::Statement(s) => s.codegen(data),
            Element::Function(f) => f.codegen(data),
            Element::Empty => String::new(),
        }
    }
}

impl Codegen for TopLevelStatement {
    fn codegen(self, data: &mut StaticData) -> String {
        match self {
            TopLevelStatement::Declaration(d) => {
                let mut code = String::new();
                code.push_str(&format!(".global {}\n", d.name));
                if let Some(v) = d.value {
                    let load = v.codegen(data);
                    data.preload_code.push_str(&load);
                    data.preload_code.push_str(&format!("cpy {} $_tmp\n", d.name));
                }
                code
            },
            TopLevelStatement::Include(i) => format!(".extern {}\n", i),
            TopLevelStatement::Use(u) => {
                let mut code = String::new();
                for s in u {
                    code.push_str(&format!(".use {}\n", s));
                }
                code
            }
        }
    }
}

impl Codegen for Function {
    fn codegen(self, data: &mut StaticData) -> String {
        let mut code = String::new();
        code.push_str(&format!("@{}:\n", self.name));
        if self.name == "main" {
            code.push_str(&format!("call {}\n", data.preload_name));
        }
        for (param, _) in self.parameters {
            code.push_str(&format!("pop {}\n", param));
        }
        for stmt in self.body.statements {
            code.push_str(&stmt.codegen(data));
        }
        if !code.ends_with("ret\n") {
            code.push_str("ret\n");
        }
        code
    }
}

impl Codegen for Statement {
    fn codegen(self, data: &mut StaticData) -> String {
        match self {
            Statement::Block(b) => {
                let mut code = String::new();
                for stmt in b.statements {
                    code.push_str(&stmt.codegen(data));
                }
                code
            }
            Statement::Expression(e) => e.codegen(data),
            Statement::Declaration(d) => {
                let mut code = String::new();
                if let Some(v) = d.value {
                    code.push_str(&v.codegen(data));
                    code.push_str(&format!("cpy {} $_tmp", d.name));
                }
                else {
                    code.push_str(&format!("cpy {} null", d.name));
                }
                code
            },
            Statement::Assignment(a) => {
                let mut code = String::new();
                code.push_str(&a.value.codegen(data));
                code.push_str(format!("cpy {} $_tmp\n", a.name).as_str());
                code
            },
            Statement::Break => {
                data.label_stack.pop().unwrap();
                let label = data.label_stack.pop().unwrap();
                format!("jmp {}\n", label)
            }
            Statement::Continue => {
                let label = data.label_stack.last().unwrap();
                format!("jmp {}\n", label)
            }
            Statement::If(i) => i.codegen(data),
            Statement::While(w) => w.codegen(data),
            Statement::For(f) => f.codegen(data),
            Statement::Return(r) => {
                let mut code = String::new();
                if let Some(v) = r {
                    code.push_str(&v.codegen(data));
                    code.push_str("push_ret $_tmp\n");
                }
                code.push_str("ret\n");
                code
            }
            Statement::Noop => String::new()
        }
    }
}

impl Codegen for IfStatement {
    fn codegen(self, data: &mut StaticData) -> String {
        let true_label = data.next_label();
        let false_label = data.next_label();
        let after_label = data.next_label();
        let cond = self.condition.codegen_conditional(data, &true_label, &false_label);
        let block = self.body.codegen(data);
        let else_block = self.else_body.map(|e| e.codegen(data)).unwrap_or(String::new());
        format!("{}.{}:\n{}jmp {}\n.{}:\n{}.{}:\n", cond, true_label, block, after_label, false_label, else_block, after_label)
    }
}

impl Codegen for WhileStatement {
    fn codegen(self, data: &mut StaticData) -> String {
        String::new()
    }
}

impl Codegen for ForStatement {
    fn codegen(self, data: &mut StaticData) -> String {
        String::new()
    }
}

impl Codegen for Expression {
    fn codegen(self, data: &mut StaticData) -> String {
        match self {
            Expression::Literal(l) => {
                format!("cpy _tmp {}\n", match l {
                    Literal::Integer(i) => i.to_string(),
                    Literal::Float(f) => f.to_string(),
                    Literal::Char(c) => format!("'{}'", c),
                    Literal::String(s) => format!("#\"{}\"", s),
                    Literal::Bool(b) => b.to_string(),
                    Literal::Null => "null".to_string()
                })
            }
            Expression::Identifier(i) => format!("cpy _tmp ${}\n", i),
            Expression::Binary(b) => {
                let mut code = String::new();
                code.push_str(&b.right.codegen(data));
                code.push_str("cpy _tmp2 $_tmp\n");
                code.push_str(&b.left.codegen(data));
                code.push_str(&format!("{} _tmp $_tmp2\n", match b.operator {
                    Operator::Plus => "add",
                    Operator::Minus => "sub",
                    Operator::Multiply => "mul",
                    Operator::Divide => "div",
                    Operator::Modulo => "mod",
                    Operator::BitwiseAnd => "and",
                    Operator::BitwiseOr => "or",
                    Operator::Xor => "xor",
                    Operator::LeftShift => "shl",
                    Operator::LogicalRightShift => "shr",
                    Operator::ArithmeticRightShift => "sar",
                    _ => panic!("Operator {} not supported or not implemented for non-conditional codegen!", b.operator)
                }));
                code
            }
            Expression::Unary(u) => {
                let mut code = String::new();
                code.push_str(&u.expr.codegen(data));
                code.push_str(match u.operator {
                    Operator::Minus => "neg _tmp\n",
                    Operator::Not => "not _tmp\n",
                    _ => panic!("Unary expression with non-unary operator {}!", u.operator)
                });
                code
            }
            Expression::Call(mut c) => {
                let mut code = String::new();
                c.arguments.reverse();
                if c.function == "print" || c.function == "sh" {
                    if c.arguments.len() != 1 {
                        panic!("Illegal number of arguments for builtin {}!", c.function);
                    }
                    code.push_str(&c.arguments.pop().unwrap().codegen(data));
                    code.push_str(&format!("{} $_tmp\n", c.function));                }
                else {
                    for arg in c.arguments {
                        code.push_str(&arg.codegen(data));
                        code.push_str("push $_tmp\n");
                    }
                    code.push_str(&format!("call {}\n", c.function));
                    code.push_str("pop_ret _tmp\n");
                }
                code
            }
            Expression::Argument(a) => {
                let mut code = String::new();
                code.push_str(&a.codegen(data));
                code.push_str("cpy _tmp %$_tmp\n");
                code
            }
        }
    }

    fn codegen_conditional(self, data: &mut StaticData, true_label: &str, false_label: &str) -> String {
        match self {
            Expression::Literal(l) => {
                match l {
                    Literal::Integer(i) => format!("jnz {} {}\njmp {}\n", i, true_label, false_label),
                    Literal::Float(f) => format!("jnz {} {}\njmp {}\n", f, true_label, false_label),
                    Literal::Char(c) => format!("jnz {} {}\njmp {}\n", c, true_label, false_label),
                    Literal::String(s) => format!("cmp {} #\"\"\njne {}\n jmp {}\n", s, true_label, false_label),
                    Literal::Bool(b) => format!("cmp {} true\nje {}\njne {}\n", b, true_label, false_label),
                    Literal::Null => format!("jmp {}\n", false_label),
                }
            }
            Expression::Identifier(i) => format!("cmp ${} true\n je {}\n, jmp {}\n", i, true_label, false_label),
            Expression::Binary(b) => {
                if b.operator == Operator::And {
                    let and_true_label = data.next_label();
                    let left_code = b.left.codegen_conditional(data, &and_true_label, false_label);
                    let right_code = b.right.codegen_conditional(data, true_label, false_label);
                    format!("{}\n.{}:\n{}\n", left_code, and_true_label, right_code)
                }
                else if b.operator == Operator::Or {
                    let or_false_label = data.next_label();
                    let left_code = b.left.codegen_conditional(data, true_label, &or_false_label);
                    let right_code = b.right.codegen_conditional(data, true_label, false_label);
                    format!("{}\n.{}:\n{}\n", left_code, or_false_label, right_code)
                }
                else {
                    let mut code = String::new();
                    if b.operator == Operator::Equal || b.operator == Operator::NotEqual {
                        let n = if b.operator == Operator::Equal { "" } else { "n" };
                        if b.left.is_null() || b.left.is_zero() {
                            code.push_str(&b.right.codegen(data));
                            code.push_str(&format!("j{}{} $_tmp {}\njmp {}\n", n, if b.left.is_null() { "n" } else { "z" }, true_label, false_label));
                        }
                        else if b.right.is_null() || b.right.is_zero() {
                            code.push_str(&b.left.codegen(data));
                            code.push_str(&format!("j{}{} $_tmp {}\njmp {}\n", n, if b.right.is_null() { "n" } else { "z" }, true_label, false_label));
                        }
                        return code;
                    }
                    code.push_str(&b.right.codegen(data));
                    code.push_str("cpy _tmp2 $_tmp\n");
                    code.push_str(&b.left.codegen(data));
                    code.push_str("cmp $_tmp2 $_tmp\n");
                    code.push_str(&format!("{} {}\n", match b.operator {
                        Operator::Equal => "je",
                        Operator::NotEqual => "jne",
                        Operator::LessThan => "jl",
                        Operator::GreaterThan => "jg",
                        Operator::LessOrEqual => "jle",
                        Operator::GreaterOrEqual => "jge",
                        _ => panic!("Operator {} not supported for conditional codegen!", b.operator)
                    }, true_label));
                    code.push_str(&format!("jmp {}\n", false_label));
                    code
                }
            }
            Expression::Unary(u) => {
                if u.operator != Operator::Not {
                    panic!("Illegal unary operator for conditional codegen!");
                }
                u.expr.codegen_conditional(data, false_label, true_label)
            }
            Expression::Call(mut c) => {
                let mut code = String::new();
                c.arguments.reverse();
                if c.function == "print" || c.function == "sh" {
                    panic!("Builtin function does not return a boolean value!");
                }
                else {
                    for arg in c.arguments {
                        code.push_str(&arg.codegen(data));
                        code.push_str("push $_tmp\n");
                    }
                    code.push_str(&format!("call {}\n", c.function));
                    code.push_str("pop_ret _tmp\n");
                }
                code.push_str("cmp $_tmp true\n");
                code.push_str(&format!("je {}\n", true_label));
                code.push_str(&format!("jmp {}\n", false_label));
                code
            }
            _ => panic!("This expression is not supported for conditional codegen!")
        }
    }
}