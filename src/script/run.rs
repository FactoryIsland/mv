use std::mem;
use std::process::Command;
use bytebuffer::ByteBuffer;
use mvutils::save::Loader;
use mvutils::unsafe_multi_borrow_mut;
use crate::script::assembly::consts::*;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

fn get_str_any(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    match ident {
        LITERAL => {
            buffer.pop_string().unwrap()
        }
        VARIABLE | REFERENCE => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Argument id {} out of range!", index));
            }
            variables[index as usize].to_string()
        }
        DEREF => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Variable id {} out of range!", index));
            }
            let reference = &variables[index as usize];
            reference.dereference().to_string()
        }
        ARGUMENT => {
            let ident = buffer.pop_u8().unwrap() as char;
            let id = if ident == VARIABLE {
                let index = buffer.pop_u32().unwrap();
                if index >= variables.len() as u32 {
                    err(format!("Argument id {} out of range!", index));
                }
                variables[index as usize].int() as usize
            }
            else {
                buffer.pop_u16().unwrap() as usize
            };
            if id as usize >= args.len() {
                err(format!("Argument id {} out of range!", id));
            }
            args[id as usize].clone()
        }
        NULL => {
            "null".to_string()
        }
        INTEGER => {
            let value = buffer.pop_i64().unwrap();
            format!("{}", value)
        }
        FLOAT => {
            let value = buffer.pop_f64().unwrap();
            format!("{}", value)
        }
        CHAR => unsafe {
            let value = buffer.pop_u32().unwrap();
            #[allow(clippy::transmute_int_to_char)]
            mem::transmute::<u32, char>(value).to_string()
        }
        BOOLEAN_TRUE => {
            "true".to_string()
        }
        BOOLEAN_FALSE => {
            "false".to_string()
        }
        _ => {
            err(format!("Unknown string identifier: {}!", ident as u8));
            String::new()
        }
    }
}

fn get_str(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    match ident {
        LITERAL => {
            buffer.pop_string().unwrap()
        }
        VARIABLE | REFERENCE => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Variable id {} out of range!", index));
            }
            variables[index as usize].not_null().string()
        }
        DEREF => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Variable id {} out of range!", index));
            }
            let reference = &variables[index as usize];
            reference.dereference().not_null().string()
        }
        ARGUMENT => {
            let ident = buffer.pop_u8().unwrap() as char;
            let id = if ident == VARIABLE {
                let index = buffer.pop_u32().unwrap();
                if index >= variables.len() as u32 {
                    err(format!("Argument id {} out of range!", index));
                }
                variables[index as usize].int() as usize
            }
            else {
                buffer.pop_u16().unwrap() as usize
            };
            if id as usize >= args.len() {
                err(format!("Argument id {} out of range!", id));
            }
            args[id as usize].clone()
        }
        _ => {
            err(format!("Unknown string identifier: {}!", ident as u8));
            String::new()
        }
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
        REFERENCE => {
            let index = buffer.pop_u32().unwrap();
            if index >= variables.len() as u32 {
                err(format!("Variable id {} out of range!", index));
            }
            Variable::Reference(&mut variables[index as usize] as *mut Variable)
        }
        DEREF => {
            let pos = buffer.get_rpos();
            let ident = buffer.pop_u8().unwrap() as char;
            if ident == REFERENCE {
                let index = buffer.pop_u32().unwrap();
                if index >= variables.len() as u32 {
                    err(format!("Variable id {} out of range!", index));
                }
                variables[index as usize].clone()
            }
            else {
                buffer.set_rpos(pos);
                let index = buffer.pop_u32().unwrap();
                if index >= variables.len() as u32 {
                    err(format!("Variable id {} out of range!", index));
                }
                let reference = &variables[index as usize];
                reference.dereference()
            }
        }
        ARGUMENT => {
            let ident = buffer.pop_u8().unwrap() as char;
            let id = if ident == VARIABLE {
                let index = buffer.pop_u32().unwrap();
                if index >= variables.len() as u32 {
                    err(format!("Argument id {} out of range!", index));
                }
                variables[index as usize].int() as usize
            }
            else {
                buffer.pop_u16().unwrap() as usize
            };
            if id as usize >= args.len() {
                err(format!("Argument id {} out of range!", id));
            }
            Variable::String(args[id as usize].clone())
        }
        NULL => {
            Variable::Null
        }
        INTEGER => {
            let value = buffer.pop_i64().unwrap();
            Variable::Int(value)
        }
        FLOAT => {
            let value = buffer.pop_f64().unwrap();
            Variable::Float(value)
        }
        CHAR => {
            let value = buffer.pop_u32().unwrap();
            Variable::Char(value)
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

fn get_variable<'a>(buffer: &mut ByteBuffer, variables: &'a mut [Variable]) -> &'a mut Variable {
    let pos = buffer.get_rpos();
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == DEREF {
        let id = buffer.pop_u32().unwrap();
        if id >= variables.len() as u32 {
            err(format!("Variable id {} out of range!", id));
        }
        variables[id as usize].dereference_ptr()
    }
    else {
        buffer.set_rpos(pos);
        let id = buffer.pop_u32().unwrap();
        if id >= variables.len() as u32 {
            err(format!("Variable id {} out of range!", id));
        }
        &mut variables[id as usize]
    }
}

fn get_jmp(buffer: &mut ByteBuffer, variables: &[Variable], addr_table: &[usize]) -> usize {
    let pos = buffer.get_rpos();
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == VARIABLE {
        let id = buffer.pop_u32().unwrap();
        if id >= variables.len() as u32 {
            err(format!("Variable id {} out of range!", id));
        }
        addr_table[variables[id as usize].as_addr()]
    }
    else {
        buffer.set_rpos(pos);
        buffer.pop_u32().unwrap() as usize
    }
}

pub fn run(code: &[u8], args: Vec<String>) {
    let mut buffer = ByteBuffer::from_bytes(code);
    let mut variables: Vec<Variable> = Vec::new();
    let mut call_stack: Vec<usize> = Vec::new();
    let mut arg_stack: Vec<Variable> = Vec::new();
    let mut addr_table: Vec<usize> = Vec::new();
    let mut cmp = Cmp::Empty;
    let mut ret = Variable::Null;

    let main = buffer.pop_u32().unwrap();
    if main >= buffer.len() as u32 {
        err(format!("Main function id {} out of range!", main));
    }

    let mut addr = buffer.pop_u32().unwrap();
    if addr > 0 {
        buffer.set_rpos(addr as usize);
        loop {
            addr = if let Some(v) = buffer.pop_u32() { v } else { break; };
            addr_table.push(addr as usize);
        }
    }

    buffer.set_rpos(main as usize);

    loop {
        let codec = buffer.pop_u8();
        if codec.is_none() { break; }
        let codec = codec.unwrap();
        match codec {
            NOOP => {}
            END => {
                let value = ret.int_or(0);
                println!("Program exited with code {}", value);
                break;
            },
            MOV => {
                let pos = buffer.get_rpos();
                let ident = buffer.pop_u8().unwrap() as char;
                if ident == DEREF {
                    let id = buffer.pop_u32().unwrap() as usize;
                    if variables.len() <= id {
                        err("Setting a pointer variable must require the variable to already exist!".to_string());
                    }
                    let variable = parse_variable(&mut buffer, &mut variables, &args, false);
                    variables[id].set_reference(variable);
                }
                else {
                    buffer.set_rpos(pos);
                    let id = buffer.pop_u32().unwrap() as usize;
                    if variables.len() <= id {
                        variables.resize(id + 1, Variable::Null);
                    }
                    let variable = parse_variable(&mut buffer, &mut variables, &args, true);
                    variables[id] = variable;
                }
            }
            JMP => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                buffer.set_rpos(addr);
            }
            JZ => {
                let value = parse_variable(&mut buffer, &mut variables, &args, false);
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if value.is_zero() {
                    buffer.set_rpos(addr);
                }
            }
            JNZ => {
                let value = parse_variable(&mut buffer, &mut variables, &args, false);
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if !value.is_zero() {
                    buffer.set_rpos(addr);
                }
            }
            JN => {
                let value = parse_variable(&mut buffer, &mut variables, &args, false);
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if value.is_null() {
                    buffer.set_rpos(addr);
                }
            }
            JNN => {
                let value = parse_variable(&mut buffer, &mut variables, &args, false);
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if !value.is_null() {
                    buffer.set_rpos(addr);
                }
            }
            CMP => {
                let a = parse_variable(&mut buffer, &mut variables, &args, false);
                let b = parse_variable(&mut buffer, &mut variables, &args, false);
                cmp = a.compare(&b);
            }
            JE => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JNE => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp != Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JG => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp == Cmp::Greater {
                    buffer.set_rpos(addr);
                }
            }
            JGE => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp == Cmp::Greater || cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            JL => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp == Cmp::Less {
                    buffer.set_rpos(addr);
                }
            }
            JLE => {
                let addr = get_jmp(&mut buffer, &variables, &addr_table);
                if cmp == Cmp::Less || cmp == Cmp::Equal {
                    buffer.set_rpos(addr);
                }
            }
            CALL => {
                let pos = buffer.get_rpos();
                let ident = buffer.pop_u8().unwrap() as char;
                if ident == BUILTIN {
                    call_function(buffer.pop_u32().unwrap(), &mut arg_stack);
                }
                else {
                    buffer.set_rpos(pos);
                    let addr = buffer.pop_u32().unwrap() as usize;
                    call_stack.push(buffer.get_rpos());
                    buffer.set_rpos(addr);
                }
            }
            RET => {
                if call_stack.is_empty() {
                    let value = ret.int_or(0);
                    println!("Program exited with code {}", value);
                    break;
                }
                let addr = call_stack.pop().unwrap();
                buffer.set_rpos(addr);
            }
            INC => {
                get_variable(&mut buffer, &mut variables).inc();
            }
            DEC => {
                get_variable(&mut buffer, &mut variables).dec();
            }
            ADD => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).add(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SUB => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).sub(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            MUL => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).mul(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            DIV => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).div(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            MOD => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).rem(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            AND => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).and(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            OR => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).or(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            NOT => {
                get_variable(&mut buffer, &mut variables).not();
            }
            NEG => {
                get_variable(&mut buffer, &mut variables).neg();
            }
            XOR => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).xor(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SHL => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).shl(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SHR => {
                get_variable(
                    &mut buffer,
                    unsafe_multi_borrow_mut!(variables, Vec<Variable>)
                ).shr(&parse_variable(&mut buffer, &mut variables, &args, false));
            }
            SAR => {
                get_variable(
                    &mut buffer,
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
            PUSH_RET => {
                ret = parse_variable(&mut buffer, &mut variables, &args, false);
            }
            POP_RET => {
                let id = buffer.pop_u32().unwrap() as usize;
                if variables.len() <= id {
                    variables.resize(id + 1, Variable::Null);
                }
                variables[id] = ret.take();
            }
            CPY => {
                let pos = buffer.get_rpos();
                let ident = buffer.pop_u8().unwrap() as char;
                if ident == DEREF {
                    let id = buffer.pop_u32().unwrap() as usize;
                    if variables.len() <= id {
                        err("Setting a pointer variable must require the variable to already exist!".to_string());
                    }
                    let variable = parse_variable(&mut buffer, &mut variables, &args, false);
                    variables[id].set_reference(variable);
                }
                else {
                    buffer.set_rpos(pos);
                    let id = buffer.pop_u32().unwrap() as usize;
                    if variables.len() <= id {
                        variables.resize(id + 1, Variable::Null);
                    }
                    let variable = parse_variable(&mut buffer, &mut variables, &args, false);
                    variables[id] = variable;
                }
            }
            _ => err(format!("Unknown codec: {}!", codec)),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
enum Variable {
    String(String),
    Char(u32),
    Int(i64),
    Float(f64),
    Bool(bool),
    Reference(*mut Variable),
    #[default]
    Null
}

impl Variable {
    fn take(&mut self) -> Variable {
        mem::replace(self, Variable::Null)
    }

    fn is_zero(&self) -> bool {
        match self {
            Variable::String(s) => s.is_empty(),
            Variable::Int(i) => *i == 0,
            Variable::Float(f) => *f == 0.0,
            Variable::Bool(b) => !*b,
            Variable::Char(c) => *c == 0,
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().is_zero() }
            Variable::Null => true
        }
    }

    fn is_null(&self) -> bool {
        match self {
            Variable::Null => true,
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().is_null() }
            _ => false
        }
    }

    fn as_addr(&self) -> usize {
        match self {
            Variable::Char(c) => *c as usize,
            Variable::Int(i) => *i as usize,
            Variable::Float(f) => *f as usize,
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().as_addr() },
            Variable::Null => 0,
            _ => {
                err("Variable is not convertible to an address!".to_string());
                0
            }
        }
    }

    fn is_reference(&self) -> bool {
        matches!(self, Variable::Reference(_))
    }

    fn set_reference(&mut self, val: Variable) {
        match self {
            Variable::Reference(ptr) => unsafe {
                (*ptr).write(val);
            }
            _ => err("Mutating reference value on non-reference!".to_string())
        }
    }

    fn dereference(&self) -> Variable {
        match self {
            Variable::Reference(ptr) => unsafe {
                (*ptr).read()
            }
            _ => {
                err("Cannot dereference a non-reference variable!".to_string());
                self.clone()
            }
        }
    }

    fn dereference_ptr(&mut self) -> &mut Variable {
        match self {
            Variable::Reference(ptr) => unsafe {
                ptr.as_mut().unwrap()
            }
            _ => {
                err("Cannot dereference a non-reference variable!".to_string());
                self
            }
        }
    }

    fn not_null(&self) -> &Variable {
        match self {
            Variable::Null => {
                err("Null variable!".to_string());
                &Variable::Null
            }
            Variable::Reference(ptr) => unsafe {
                (*ptr).as_ref().unwrap().not_null()
            }
            _ => self
        }
    }

    fn char(&self) -> char {
        match self {
            #[allow(clippy::transmute_int_to_char)]
            Variable::Char(c) => unsafe { mem::transmute(*c) },
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().char() }
            Variable::Null => '\0',
            _ => {
                err("Variable is not a char!".to_string());
                '\0'
            }
        }
    }

    fn string(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            Variable::Null => "null".to_string(),
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().string() }
            _ => {
                err("Variable is not a string!".to_string());
                String::new()
            }
        }
    }

    fn string_or_char(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            #[allow(clippy::transmute_int_to_char)]
            Variable::Char(c) => unsafe { mem::transmute::<u32, char>(*c).to_string() },
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().string_or_char() }
            Variable::Null => "null".to_string(),
            _ => {
                err("Variable is not a string or char!".to_string());
                String::new()
            }
        }
    }

    fn int(&self) -> i64 {
        match self {
            Variable::Int(i) => *i,
            Variable::Null => 0,
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().int() }
            _ => {
                err("Variable is not an integer!".to_string());
                0
            }
        }
    }

    fn int_or(&self, value: i64) -> i64 {
        match self {
            Variable::Int(i) => *i,
            _ => value
        }
    }

    fn float(&self) -> f64 {
        match self {
            Variable::Float(f) => *f,
            Variable::Null => 0.0,
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().float() },
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
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().bool() }
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
                Variable::Reference(ptr) => {
                    self.compare(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => {
                    err("Cannot compare string with other types!".to_string());
                    Cmp::Empty
                }
            }
            Variable::Char(a) => match other {
                Variable::Char(b) => {
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
                Variable::Int(b) => {
                    let b = *b as u32;
                    let a = *a;
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
                Variable::Reference(ptr) => {
                    self.compare(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => {
                    err("Cannot compare string with non number types!".to_string());
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
                Variable::Char(b) => {
                    let b = *b as i64;
                    let a = *a;
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
                Variable::Reference(ptr) => {
                    self.compare(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => {
                    err("Cannot compare string with non number types!".to_string());
                    Cmp::Empty
                }
            }
            Variable::Float(a) => match other {
                Variable::Float(b) => cmp_float(*a, *b),
                Variable::Int(b) => cmp_float(*a, *b as f64),
                Variable::Null => Cmp::NotEqual,
                Variable::Reference(ptr) => {
                    self.compare(unsafe { (*ptr).as_ref().unwrap() })
                }
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
                Variable::Reference(ptr) => {
                    self.compare(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => Cmp::NotEqual
            }
            Variable::Null => {
                match other {
                    Variable::Null => Cmp::Equal,
                    Variable::Reference(ptr) => {
                        self.compare(unsafe { (*ptr).as_ref().unwrap() })
                    }
                    _ => Cmp::NotEqual
                }
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().compare(other) }
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
            Variable::Char(c) => {
                *c += 1;
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().inc() }
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
            Variable::Char(c) => {
                *c -= 1;
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().dec() }
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
                Variable::Char(b) => {
                    *a += *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.add(unsafe { (*ptr).as_ref().unwrap() })
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
                Variable::Char(b) => {
                    *a += *b as f64;
                }
                Variable::Reference(ptr) => {
                    self.add(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot add non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a += *b as u32;
                }
                Variable::Float(b) => {
                    *a += *b as u32;
                }
                Variable::Char(b) => {
                    *a += *b;
                }
                Variable::Reference(ptr) => {
                    self.add(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot add non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().add(other) }
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
                Variable::Char(b) => {
                    *a -= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.sub(unsafe { (*ptr).as_ref().unwrap() })
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
                Variable::Char(b) => {
                    *a -= *b as f64;
                }
                Variable::Reference(ptr) => {
                    self.sub(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot subtract non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a -= *b as u32;
                }
                Variable::Float(b) => {
                    *a -= *b as u32;
                }
                Variable::Char(b) => {
                    *a -= *b;
                }
                Variable::Reference(ptr) => {
                    self.sub(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot subtract non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().sub(other) }
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
                Variable::Char(b) => {
                    *a *= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.mul(unsafe { (*ptr).as_ref().unwrap() })
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
                Variable::Char(b) => {
                    *a *= *b as f64;
                }
                Variable::Reference(ptr) => {
                    self.mul(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot multiply non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a *= *b as u32;
                }
                Variable::Float(b) => {
                    *a *= *b as u32;
                }
                Variable::Char(b) => {
                    *a *= *b;
                }
                Variable::Reference(ptr) => {
                    self.mul(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot multiply non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().mul(other) }
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
                Variable::Char(b) => {
                    *a /= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.div(unsafe { (*ptr).as_ref().unwrap() })
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
                Variable::Char(b) => {
                    *a /= *b as f64;
                }
                Variable::Reference(ptr) => {
                    self.div(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot divide non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a /= *b as u32;
                }
                Variable::Float(b) => {
                    *a /= *b as u32;
                }
                Variable::Char(b) => {
                    *a /= *b;
                }
                Variable::Reference(ptr) => {
                    self.div(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot divide non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().div(other) }
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
                Variable::Char(b) => {
                    *a %= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.rem(unsafe { (*ptr).as_ref().unwrap() })
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
                Variable::Char(b) => {
                    *a %= *b as f64;
                }
                Variable::Reference(ptr) => {
                    self.rem(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot modulo non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a %= *b as u32;
                }
                Variable::Float(b) => {
                    *a %= *b as u32;
                }
                Variable::Char(b) => {
                    *a %= *b;
                }
                Variable::Reference(ptr) => {
                    self.rem(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot modulo non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().rem(other) }
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
                Variable::Char(b) => {
                    *a &= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.and(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot and non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a &= *b as u32;
                }
                Variable::Float(b) => {
                    *a &= *b as u32;
                }
                Variable::Char(b) => {
                    *a &= *b;
                }
                Variable::Reference(ptr) => {
                    self.and(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot and non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a &= *b;
                }
                Variable::Reference(ptr) => {
                    self.and(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot and non boolean types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().and(other) }
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
                Variable::Char(b) => {
                    *a |= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.or(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot or non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a |= *b as u32;
                }
                Variable::Float(b) => {
                    *a |= *b as u32;
                }
                Variable::Char(b) => {
                    *a |= *b;
                }
                Variable::Reference(ptr) => {
                    self.or(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot or non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a |= *b;
                }
                Variable::Reference(ptr) => {
                    self.or(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot or non boolean types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().or(other) }
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
                Variable::Char(b) => {
                    *a ^= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.xor(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot xor non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a ^= *b as u32;
                }
                Variable::Float(b) => {
                    *a ^= *b as u32;
                }
                Variable::Char(b) => {
                    *a ^= *b;
                }
                Variable::Reference(ptr) => {
                    self.xor(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot xor non number types!".to_string())
            }
            Variable::Bool(a) => match other {
                Variable::Bool(b) => {
                    *a ^= *b;
                }
                Variable::Reference(ptr) => {
                    self.xor(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot xor non boolean types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().xor(other) }
            _ => err("Cannot xor non boolean or non numeric types!".to_string())
        }
    }

    fn not(&mut self) {
        match self {
            Variable::Int(a) => {
                *a = !*a;
            }
            Variable::Char(a) => {
                *a = !*a;
            }
            Variable::Bool(a) => {
                *a = !*a;
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().not() }
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
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().neg() }
            _ => err("Cannot neg non numeric types!".to_string())
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
                Variable::Char(b) => {
                    *a <<= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.shl(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot shl non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a <<= *b as u32;
                }
                Variable::Float(b) => {
                    *a <<= *b as u32;
                }
                Variable::Char(b) => {
                    *a <<= *b;
                }
                Variable::Reference(ptr) => {
                    self.shl(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot shl non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().shl(other) }
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
                Variable::Char(b) => {
                    *a = ((*a as u64) >> *b as i64) as i64;
                }
                Variable::Reference(ptr) => {
                    self.shr(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot shr non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a >>= *b as u32;
                }
                Variable::Float(b) => {
                    *a >>= *b as u32;
                }
                Variable::Char(b) => {
                    *a >>= *b;
                }
                Variable::Reference(ptr) => {
                    self.shr(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot shr non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().shr(other) }
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
                Variable::Char(b) => {
                    *a >>= *b as i64;
                }
                Variable::Reference(ptr) => {
                    self.sar(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot sar non number types!".to_string())
            }
            Variable::Char(a) => match other {
                Variable::Int(b) => {
                    *a >>= *b as u32;
                }
                Variable::Float(b) => {
                    *a >>= *b as u32;
                }
                Variable::Char(b) => {
                    *a >>= *b;
                }
                Variable::Reference(ptr) => {
                    self.sar(unsafe { (*ptr).as_ref().unwrap() })
                }
                _ => err("Cannot sar non number types!".to_string())
            }
            Variable::Reference(ptr) => unsafe { (*ptr).as_mut().unwrap().sar(other) }
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
            #[allow(clippy::transmute_int_to_char)]
            Variable::Char(c) => unsafe { mem::transmute::<u32, char>(*c).to_string() },
            Variable::Int(i) => format!("{}", i),
            Variable::Float(f) => format!("{}", f),
            Variable::Bool(b) => format!("{}", b),
            Variable::Reference(ptr) => unsafe { (*ptr).as_ref().unwrap().to_string() }
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

fn call_function(id: u32, stack: &mut Vec<Variable>) {
    match id {
        GIT_ADD_ALL => {
            Command::new("git").arg("add").arg("*").spawn().unwrap().wait().unwrap();
        }
        GIT_ADD => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("add").arg(str).spawn().unwrap().wait().unwrap();
        }
        GIT_COMMIT_DEFAULT => {
            Command::new("git").arg("commit").arg("-m").arg("").spawn().unwrap().wait().unwrap();
        }
        GIT_COMMIT => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("commit").arg("-m").arg(str).spawn().unwrap().wait().unwrap();
        }
        GIT_PUSH_UPSTREAM => {
            Command::new("git").arg("push").spawn().unwrap().wait().unwrap();
        }
        GIT_PUSH => {
            let str = stack.pop().unwrap().not_null().string();
            Command::new("git").arg("push").arg("-u").args(str.split(' ')).spawn().unwrap().wait().unwrap();
        }
        _ => {
            err(format!("Unknown built-in function id: {}", id));
        }
    }
}