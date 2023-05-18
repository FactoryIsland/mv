use std::process::Command;
use bytebuffer::ByteBuffer;
use mvutils::save::Loader;
use mvutils::unsafe_multi_borrow_mut;
use crate::script::consts::*;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

fn get_str_any(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == LITERAL {
        buffer.pop_string().unwrap()
    }
    else if ident == VARIABLE {
        let index = buffer.pop_u32().unwrap();
        if index >= variables.len() as u32 {
            err(format!("Argument id {} out of range!", index));
        }
        variables[index as usize].to_string()
    }
    else if ident == ARGUMENT {
        let id = buffer.pop_u32().unwrap();
        if id >= args.len() as u32 {
            err(format!("Argument id {} out of range!", id));
        }
        args[id as usize].clone()
    }
    else {
        err(format!("Unknown string identifier: {}!", ident as u8));
        String::new()
    }
}

fn get_str(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == LITERAL {
        buffer.pop_string().unwrap()
    }
    else if ident == VARIABLE {
        let index = buffer.pop_u32().unwrap();
        if index >= variables.len() as u32 {
            err(format!("Variable id {} out of range!", index));
        }
        let var = &variables[index as usize];
        if let Variable::String(s) = var {
            s.clone()
        }
        else {
            err(format!("Variable must be a string"));
            String::new()
        }
    }
    else if ident == ARGUMENT {
        let id = buffer.pop_u32().unwrap();
        if id >= args.len() as u32 {
            err(format!("Argument id {} out of range!", id));
        }
        args[id as usize].clone()
    }
    else {
        err(format!("Unknown string identifier: {}!", ident as u8));
        String::new()
    }
}

fn parse_variable(buffer: &mut ByteBuffer, variables: &mut [Variable], args: &[String], take: bool) -> Variable {
    let ident = buffer.pop_u8().unwrap() as char;
    match ident {
        LITERAL => Variable::String(buffer.pop_string().unwrap()),
        VARIABLE => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Variable id {} out of range!", index));
            }
            if take {
                variables[index as usize].take()
            }
            else {
                variables[index as usize].clone()
            }
        }
        ARGUMENT => {
            let id = buffer.pop_u32().unwrap();
            if id >= args.len() as u32 {
                err(format!("Argument id {} out of range!", id));
            }
            Variable::String(args[id as usize].clone())
        }
        INTEGER => {
            let value = buffer.pop_i64().unwrap();
            Variable::Int(value)
        }
        FLOAT => {
            let value = buffer.pop_f64().unwrap();
            Variable::Float(value)
        }
        BOOLEAN_TRUE => {
            Variable::Bool(true)
        }
        BOOLEAN_FALSE => {
            Variable::Bool(false)
        }
        _ => {
            err(format!("Unknown variable identifier: {}!", ident as u8));
            Variable::Null
        }
    }
}

fn get_variable(id: u32, variables: &mut [Variable]) -> &mut Variable {
    if id >= variables.len() as u32 {
        err(format!("Variable id {} out of range!", id));
    }
    &mut variables[id as usize]
}

pub fn run(code: &[u8], args: Vec<String>) {
    let mut buffer = ByteBuffer::from_bytes(code);
    let mut variables: Vec<Variable> = Vec::new();
    let mut call_stack: Vec<usize> = Vec::new();
    let mut arg_stack: Vec<Variable> = Vec::new();
    let mut cmp = Cmp::Empty;

    let main = buffer.pop_u32().unwrap();
    if main >= buffer.len() as u32 {
        err(format!("Main function id {} out of range!", main));
    }

    buffer.set_rpos(main as usize);

    loop {
        let codec = buffer.pop_u8();
        if codec.is_none() { break; }
        let codec = codec.unwrap();
        match codec {
            NOOP => {}
            END => break,
            MOV => {
                let id = buffer.pop_u32().unwrap() as usize;
                if variables.len() <= id {
                    variables.resize(id + 1, Variable::Null);
                }
                let variable = parse_variable(&mut buffer, &mut variables, &args, true);
                variables[id] = variable;
            }
            JMP => {
                let addr = buffer.pop_u32().unwrap() as usize;
                buffer.set_rpos(addr);
            }
            JZ => {
                let value = parse_variable(&mut buffer, &mut variables, &args, false);
                let addr = buffer.pop_u32().unwrap() as usize;
                if value.is_zero() {
                    buffer.set_rpos(addr);
                }
            }
            CMP => {
                let a = parse_variable(&mut buffer, &mut variables, &args, false);
                let b = parse_variable(&mut buffer, &mut variables, &args, false);
                cmp = a.compare(&b);
            }
            JE => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JNE => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp != Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JG => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp == Cmp::Greater {
                    buffer.set_rpos(addr);
                }
            }
            JGE => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp == Cmp::Greater || cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JL => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp == Cmp::Less {
                    buffer.set_rpos(addr);
                }
            }
            JLE => {
                let addr = buffer.pop_u32().unwrap() as usize;
                if cmp == Cmp::Less || cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            CALL => {
                let pos = buffer.get_rpos();
                let ident = buffer.pop_u8().unwrap() as char;
                if ident == BUILTIN {
                    let name = buffer.pop_string().unwrap();
                    call_function(name, &mut arg_stack);
                }
                else {
                    buffer.set_rpos(pos);
                    let addr = buffer.pop_u32().unwrap() as usize;
                    call_stack.push(buffer.get_rpos());
                    buffer.set_rpos(addr);
                }
            }
            RET => {
                if call_stack.len() == 0 {
                    break;
                }
                let addr = call_stack.pop().unwrap();
                buffer.set_rpos(addr);
            }
            INC => {
                get_variable(buffer.pop_u32().unwrap(), &mut variables).inc();
            }
            DEC => {
                get_variable(buffer.pop_u32().unwrap(), &mut variables).dec();
            }
            ADD => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).add(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SUB => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).sub(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            MUL => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).mul(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            DIV => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).div(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            MOD => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).rem(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            AND => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).and(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            OR => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).or(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            NOT => {
                get_variable(buffer.pop_u32().unwrap(), &mut variables).not();
            }
            NEG => {
                get_variable(buffer.pop_u32().unwrap(), &mut variables).neg();
            }
            XOR => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).xor(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SHL => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).shl(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SHR => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).shr(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SAR => {
                get_variable(
                    buffer.pop_u32().unwrap(),
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).sar(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            PUSH => {
                arg_stack.push(parse_variable(&mut buffer, &mut variables, &args, false));
            }
            POP => {
                let id = buffer.pop_u32().unwrap() as usize;
                if variables.len() <= id {
                    variables.resize(id + 1, Variable::Null);
                }
                variables[id] = arg_stack.pop().unwrap();
            }
            PRINT => {
                let str = get_str_any(&mut buffer, &args, &variables);
                println!("{}", str);
            }
            SH => {
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("sh").arg("-c").arg(format!("\"{}\"", str)).spawn().unwrap().wait().unwrap();
            }
            _ => err(format!("Unknown codec: {}!", codec)),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
enum Variable {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    #[default]
    Null
}

impl Variable {
    fn take(&mut self) -> Variable {
        std::mem::replace(self, Variable::Null)
    }

    fn is_zero(&self) -> bool {
        match self {
            Variable::String(s) => s.is_empty(),
            Variable::Int(i) => *i == 0,
            Variable::Float(f) => *f == 0.0,
            Variable::Bool(b) => !*b,
            Variable::Null => true
        }
    }

    fn not_null(&self) -> &Variable {
        match self {
            Variable::Null => {
                err("Null variable!".to_string());
                &Variable::Null
            }
            _ => self
        }
    }

    fn string(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            Variable::Null => "null".to_string(),
            _ => {
                err("Variable is not a string!".to_string());
                String::new()
            }
        }
    }

    fn int(&self) -> i64 {
        match self {
            Variable::Int(i) => *i,
            Variable::Null => 0,
            _ => {
                err("Variable is not an integer!".to_string());
                0
            }
        }
    }

    fn float(&self) -> f64 {
        match self {
            Variable::Float(f) => *f,
            Variable::Null => 0.0,
            _ => {
                err("Variable is not a float!".to_string());
                0.0
            }
        }
    }

    fn bool(&self) -> bool {
        match self {
            Variable::Bool(b) => *b,
            Variable::Null => false,
            _ => {
                err("Variable is not a boolean!".to_string());
                false
            }
        }
    }

    fn compare(&self, other: &Variable) -> Cmp {
        match self {
            Variable::String(a) => match other {
                Variable::String(b) => {
                    if a == b {
                        Cmp::Equal
                    }
                    else {
                        Cmp::NotEqual
                    }
                }
                Variable::Null => Cmp::NotEqual,
                _ => {
                    err("Cannot compare string with other types!".to_string());
                    Cmp::Empty
                }
            }
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    if a == b {
                        Cmp::Equal
                    }
                    else if a < b {
                        Cmp::Less
                    }
                    else {
                        Cmp::Greater
                    }
                }
                Variable::Float(b) => cmp_float(*a as f64, *b),
                Variable::Null => Cmp::NotEqual,
                _ => {
                    err("Cannot compare string with non number types!".to_string());
                    Cmp::Empty
                }
            }
            Variable::Float(a) => match other {
                Variable::Float(b) => cmp_float(*a, *b),
                Variable::Int(b) => cmp_float(*a, *b as f64),
                Variable::Null => Cmp::NotEqual,
                _ => {
                    err("Cannot compare string with non number types!".to_string());
                    Cmp::Empty
                }
            }
            Variable::Bool(b1) => match other {
                Variable::Bool(b2) => {
                    if *b1 == *b2 {
                        Cmp::Equal
                    }
                    else {
                        Cmp::NotEqual
                    }
                }
                Variable::Null => Cmp::NotEqual,
                _ => Cmp::NotEqual
            }
            Variable::Null => {
                match other {
                    Variable::Null => Cmp::Equal,
                    _ => Cmp::NotEqual
                }
            }
        }
    }

    fn inc(&mut self) {
        match self {
            Variable::Int(i) => {
                *i += 1;
            }
            Variable::Float(f) => {
                *f += 1.0;
            }
            _ => err("Cannot increment non number types!".to_string())
        }
    }

    fn dec(&mut self) {
        match self {
            Variable::Int(i) => {
                *i -= 1;
            }
            Variable::Float(f) => {
                *f -= 1.0;
            }
            _ => err("Cannot decrement non number types!".to_string())
        }
    }

    fn add(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a += *b;
                }
                Variable::Float(b) => {
                    *a += *b as i64;
                }
                _ => err("Cannot add non number types!".to_string())
            }
            Variable::Float(a) => match other {
                Variable::Int(b) => {
                    *a += *b as f64;
                }
                Variable::Float(b) => {
                    *a += *b;
                }
                _ => err("Cannot add non number types!".to_string())
            }
            _ => err("Cannot add non number types!".to_string())
        }
    }

    fn sub(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a -= *b;
                }
                Variable::Float(b) => {
                    *a -= *b as i64;
                }
                _ => err("Cannot subtract non number types!".to_string())
            }
            Variable::Float(a) => match other {
                Variable::Int(b) => {
                    *a -= *b as f64;
                }
                Variable::Float(b) => {
                    *a -= *b;
                }
                _ => err("Cannot subtract non number types!".to_string())
            }
            _ => err("Cannot subtract non number types!".to_string())
        }
    }

    fn mul(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a *= *b;
                }
                Variable::Float(b) => {
                    *a *= *b as i64;
                }
                _ => err("Cannot multiply non number types!".to_string())
            }
            Variable::Float(a) => match other {
                Variable::Int(b) => {
                    *a *= *b as f64;
                }
                Variable::Float(b) => {
                    *a *= *b;
                }
                _ => err("Cannot multiply non number types!".to_string())
            }
            _ => err("Cannot multiply non number types!".to_string())
        }
    }

    fn div(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a /= *b;
                }
                Variable::Float(b) => {
                    *a /= *b as i64;
                }
                _ => err("Cannot divide non number types!".to_string())
            }
            Variable::Float(a) => match other {
                Variable::Int(b) => {
                    *a /= *b as f64;
                }
                Variable::Float(b) => {
                    *a /= *b;
                }
                _ => err("Cannot divide non number types!".to_string())
            }
            _ => err("Cannot divide non number types!".to_string())
        }
    }

    fn rem(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a %= *b;
                }
                Variable::Float(b) => {
                    *a %= *b as i64;
                }
                _ => err("Cannot modulo non number types!".to_string())
            }
            Variable::Float(a) => match other {
                Variable::Int(b) => {
                    *a %= *b as f64;
                }
                Variable::Float(b) => {
                    *a %= *b;
                }
                _ => err("Cannot modulo non number types!".to_string())
            }
            _ => err("Cannot modulo non number types!".to_string())
        }
    }

    fn and(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a &= *b;
                }
                Variable::Float(b) => {
                    *a &= *b as i64;
                }
                _ => err("Cannot and non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a &= *b;
                }
                _ => err("Cannot and non boolean types!".to_string())
            }
            _ => err("Cannot and non boolean or non numeric types!".to_string())
        }
    }

    fn or(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a |= *b;
                }
                Variable::Float(b) => {
                    *a |= *b as i64;
                }
                _ => err("Cannot or non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a |= *b;
                }
                _ => err("Cannot or non boolean types!".to_string())
            }
            _ => err("Cannot or non boolean or non numeric types!".to_string())
        }
    }

    fn xor(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a ^= *b;
                }
                Variable::Float(b) => {
                    *a ^= *b as i64;
                }
                _ => err("Cannot xor non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a ^= *b;
                }
                _ => err("Cannot xor non boolean types!".to_string())
            }
            _ => err("Cannot xor non boolean or non numeric types!".to_string())
        }
    }

    fn not(&mut self) {
        match self {
            Variable::Int(a) => {
                *a = !*a;
            }
            Variable::Bool(a) => {
                *a = !*a;
            }
            _ => err("Cannot not non boolean or non numeric types!".to_string())
        }
    }

    fn neg(&mut self) {
        match self {
            Variable::Int(a) => {
                *a = -*a;
            }
            Variable::Float(a) => {
                *a = -*a;
            }
            _ => err("Cannot not non boolean or non numeric types!".to_string())
        }
    }

    fn shl(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a <<= *b;
                }
                Variable::Float(b) => {
                    *a <<= *b as i64;
                }
                _ => err("Cannot shl non number types!".to_string())
            }
            _ => err("Cannot shl non number types!".to_string())
        }
    }

    fn shr(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a = ((*a as u64) >> *b) as i64;
                }
                Variable::Float(b) => {
                    *a = ((*a as u64) >> *b as i64) as i64;
                }
                _ => err("Cannot shr non number types!".to_string())
            }
            _ => err("Cannot shr non number types!".to_string())
        }
    }

    fn sar(&mut self, other: &Variable) {
        match self {
            Variable::Int(a) => match other {
                Variable::Int(b) => {
                    *a >>= *b;
                }
                Variable::Float(b) => {
                    *a >>= *b as i64;
                }
                _ => err("Cannot sar non number types!".to_string())
            }
            _ => err("Cannot sar non number types!".to_string())
        }
    }
}

fn cmp_float(a: f64, b: f64) -> Cmp {
    if a == b {
        Cmp::Equal
    }
    else if a < b {
        Cmp::Less
    }
    else if a > b {
        Cmp::Greater
    }
    else {
        Cmp::NotEqual
    }
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            Variable::Int(i) => format!("{}", i),
            Variable::Float(f) => format!("{}", f),
            Variable::Bool(b) => format!("{}", b),
            Variable::Null => "null".to_string()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
enum Cmp {
    #[default]
    Empty,
    NotEqual,
    Equal,
    Greater,
    Less,
}

fn call_function(name: String, stack: &mut Vec<Variable>) {
    match name.as_str() {
        "GIT_ADD_ALL" => {
            Command::new("git").arg("add").arg("*").spawn().unwrap().wait().unwrap();
        }
        "GIT_ADD" => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("add").arg(str).spawn().unwrap().wait().unwrap();
        }
        "GIT_COMMIT_DEFAULT" => {
            Command::new("git").arg("commit").arg("-m").arg("\"\"").spawn().unwrap().wait().unwrap();
        }
        "GIT_COMMIT" => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("commit").arg("-m").arg(format!("\"{}\"", str)).spawn().unwrap().wait().unwrap();
        }
        "GIT_PUSH_UPSTREAM" => {
            Command::new("git").arg("push").spawn().unwrap().wait().unwrap();
        }
        "GIT_PUSH" => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("push").arg("-u").args(str.split(" ")).spawn().unwrap().wait().unwrap();
        }
        _ => {
            err(format!("Unknown function: {}", name));
        }
    }
}