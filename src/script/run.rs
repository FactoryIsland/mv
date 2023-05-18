use std::fs::{OpenOptions, remove_file};
use std::io::{Error, Write};
use std::process::Command;
use bytebuffer::ByteBuffer;
use mvutils::save::Loader;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

fn get_str_any(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == '#' {
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
    }
}

fn get_str(buffer: &mut ByteBuffer, args: &[String], variables: &[Variable]) -> String {
    let ident = buffer.pop_u8().unwrap() as char;
    if ident == '#' {
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
            err(format!("Variable must be a string"));
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
    }
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
            20 => { //display
                let str = get_str_any(&mut buffer, &args, &variables);
                println!("{}", str);
            }
            21 => { //sh
                let str = get_str(&mut buffer, &args, &variables);
                let mut file = OpenOptions::new().write(true).append(true).create(true).open("tmp.sh").unwrap();
                file.write_all(str.as_bytes()).unwrap();
                file.flush().unwrap();
                Command::new("sh").arg("tmp.sh").spawn().unwrap().wait().unwrap();
                remove_file("tmp.sh").unwrap();
            }
            22 => { //git add all
                Command::new("git").arg("add").arg("*").spawn().unwrap().wait().unwrap();
            }
            23 => { //git add
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("add").arg(str).spawn().unwrap().wait().unwrap();
            }
            24 => { //git commit default
                Command::new("git").arg("commit").arg("-m").arg("\"Commit\"").spawn().unwrap().wait().unwrap();
            }
            25 => { //git commit
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("commit").arg("-m").arg(format!("\"{}\"", str)).spawn().unwrap().wait().unwrap();
            }
            26 => { //git push to upstream
                Command::new("git").arg("push").spawn().unwrap().wait().unwrap();
            }
            27 => { //git push
                let str = get_str(&mut buffer, &args, &variables);
                Command::new("git").arg("push").args(str.split(" ")).spawn().unwrap().wait().unwrap();
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