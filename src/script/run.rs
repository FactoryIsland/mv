use std::fs::{OpenOptions, remove_file};
use std::io::{Error, Write};
use std::process::Command;
use bytebuffer::ByteBuffer;
use mvutils::save::Loader;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

pub fn run_mvb(code: &[u8], args: Vec<String>) {
    let mut buffer = ByteBuffer::from_bytes(code);
    let mut stack: Vec<u8> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();
    loop {
        let codec = buffer.pop_u8();
        if codec.is_none() { break; }
        let codec = codec.unwrap();
        match codec {
            0 => {} //noop
            89 => { //sh
                let ident = buffer.pop_u8().unwrap() as char;
                let str = if ident == '#' {
                    //literal
                    buffer.pop_string().unwrap()
                }
                else if ident == '$' {
                    //variable
                    let index = buffer.pop_u32().unwrap();
                    if index >= variables.len() as u32 {
                        err(format!("Argument id {} out of range!", index));
                    }
                    let var = &variables[index as usize];
                    if let Variable::String(s) = var {
                        s.clone()
                    }
                    else {
                        err(format!("Variable passed into 'sh' must be a string"));
                        String::new()
                    }
                }
                else if ident == '%' {
                    //argument
                    let id = buffer.pop_u32().unwrap();
                    if id >= args.len() as u32 {
                        err(format!("Argument id {} out of range!", id));
                    }
                    args[id as usize].clone()
                }
                else {
                    err(format!("Unknown string identifier: {}!", ident as u8));
                    String::new()
                };
                let mut file = OpenOptions::new().write(true).append(true).create(true).open("tmp.sh").unwrap();
                file.write_all(str.as_bytes()).unwrap();
                file.flush().unwrap();
                Command::new("sh").arg("tmp.sh").spawn().unwrap().wait().unwrap();
                remove_file("tmp.sh").unwrap();
            }
            90 => { //display
                let ident = buffer.pop_u8().unwrap() as char;
                let str = if ident == '#' {
                    //literal
                    buffer.pop_string().unwrap()
                }
                else if ident == '$' {
                    //variable
                    let index = buffer.pop_u32().unwrap();
                    if index >= variables.len() as u32 {
                        err(format!("Argument id {} out of range!", index));
                    }
                    variables[index as usize].to_string()
                }
                else if ident == '%' {
                    //argument
                    let id = buffer.pop_u32().unwrap();
                    if id >= args.len() as u32 {
                        err(format!("Argument id {} out of range!", id));
                    }
                    args[id as usize].clone()
                }
                else {
                    err(format!("Unknown string identifier: {}!", ident as u8));
                    String::new()
                };
                println!("{}", str);
            }
            _ => err(format!("Unknown codec: {}!", codec)),
        }
    }
}

enum Variable {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    None
}

impl Variable {
    pub fn to_string(&self) -> String {
        match self {
            Variable::String(s) => s.clone(),
            Variable::Int(i) => format!("{}", i),
            Variable::Float(f) => format!("{}", f),
            Variable::Bool(b) => format!("{}", b),
            _ => "null".to_string()
        }
    }
}