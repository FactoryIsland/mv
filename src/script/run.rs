use std::process::Command;
use bytebuffer::ByteBuffer;
use mvutils::save::Loader;
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
            let value = buffer.pop_i32().unwrap();
            Variable::Int(value)
        }
        FLOAT => {
            let value = buffer.pop_f32().unwrap();
            Variable::Float(value)
        }
        BOOLEAN => {
            let value = buffer.pop_u8().unwrap() != 0;
            Variable::Bool(value)
        }
        _ => {
            err(format!("Unknown variable identifier: {}!", ident as u8));
            Variable::Null
        }
    }
}

pub fn run_mvb(code: &[u8], args: Vec<String>) {
    let mut buffer = ByteBuffer::from_bytes(code);
    let mut variables: Vec<Variable> = Vec::new();
    loop {
        let codec = buffer.pop_u8();
        if codec.is_none() { break; }
        let codec = codec.unwrap();
        match codec {
            NOOP => {}
            END => break,
            MOV => {
                let id = buffer.pop_u32().unwrap() as usize;
                while variables.len() >= id {
                    variables.push(Variable::Null);
                }
                let variable = parse_variable(&mut buffer, &mut variables, &args, true);
                variables[id] = variable;
            }
            JMP => {
                let addr = buffer.pop_u32().unwrap() as usize;
                buffer.set_rpos(addr);
            }
            PRINT => {
                let str = get_str_any(&mut buffer, &args, &variables);
                println!("{}", str);
            }
            SH => {
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("sh").arg("-c").arg(format!("\"{}\"", str)).spawn().unwrap().wait().unwrap();
            }
            GIT_ADD_ALL => {
                Command::new("git").arg("add").arg("*").spawn().unwrap().wait().unwrap();
            }
            GIT_ADD => {
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("add").arg(str).spawn().unwrap().wait().unwrap();
            }
            GIT_COMMIT_DEFAULT => {
                Command::new("git").arg("commit").arg("-m").arg("\"\"").spawn().unwrap().wait().unwrap();
            }
            GIT_COMMIT => {
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("commit").arg("-m").arg(format!("\"{}\"", str)).spawn().unwrap().wait().unwrap();
            }
            GIT_PUSH_UPSTREAM => {
                Command::new("git").arg("push").spawn().unwrap().wait().unwrap();
            }
            GIT_PUSH => {
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("push").args(str.split(" ")).spawn().unwrap().wait().unwrap();
            }
            _ => err(format!("Unknown codec: {}!", codec)),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
enum Variable {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    #[default]
    Null
}

impl Variable {
    pub fn to_string(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            Variable::Int(i) => format!("{}", i),
            Variable::Float(f) => format!("{}", f),
            Variable::Bool(b) => format!("{}", b),
            Variable::Null => "null".to_string()
        }
    }

    pub fn take(&mut self) -> Variable {
        std::mem::replace(self, Variable::Null)
    }
}