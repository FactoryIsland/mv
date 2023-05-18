use std::collections::HashMap;
use bytebuffer::ByteBuffer;
use mvutils::save::{Loader, Saver};
use mvutils::utils::{remove_quotes, format_escaped};
use crate::script::consts::*;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

static mut NAMED: bool = false;

macro_rules! named_var {
    ($names:ident, $buffer:ident, $token:ident, $next:ident) => {
        if unsafe { NAMED } {
            if $names.contains_key($token) {
                $buffer.push_u32(*$names.get($token).unwrap());
            }
            else {
                $names.insert($token.to_string(), *$next);
                $buffer.push_u32(*$next);
                *$next += 1;
            }
        }
        else {
            $buffer.push_u32($token.parse::<u32>().unwrap());
        }
    };
}

fn push_str_var(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32) -> u32 {
    let mut chars = token.chars();
    let ident = chars.next().unwrap();
    let str = chars.as_str();
    buffer.push_u8(ident as u8);
    let mut offset = 1;
    match ident {
        LITERAL => {
            let str = format_escaped(str);
            buffer.push_string(&str);
            offset += str.len() as u32 + 4;
        },
        VARIABLE | ARGUMENT => {
            named_var!(names, buffer, str, next_var);
            offset += 4;
        },
        _ => err(format!("Invalid string identifier: {}", ident))
    }
    offset
}

fn push_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            buffer.push_u8(LITERAL as u8);
            let str = format_escaped(token.split_at(1).1);
            buffer.push_string(&str);
            str.len() as u32 + 5
        }
        VARIABLE | ARGUMENT => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var);
            5
        }
        _ => {
            if token == "true" {
                buffer.push_u8(BOOLEAN_TRUE as u8);
                1
            } else if token == "false" {
                buffer.push_u8(BOOLEAN_FALSE as u8);
                1
            } else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                } else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                5
            }
        }
    }
}

fn push_prim_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            err("Argument cannot be of type string!".to_string());
            0
        }
        VARIABLE | ARGUMENT => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var);
            5
        }
        _ => {
            if token == "true" {
                buffer.push_u8(BOOLEAN_TRUE as u8);
                1
            } else if token == "false" {
                buffer.push_u8(BOOLEAN_FALSE as u8);
                1
            } else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                } else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                5
            }
        }
    }
}

fn push_num_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            err("Argument cannot be of type string!".to_string());
            0
        }
        VARIABLE | ARGUMENT => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var);
            5
        }
        _ => {
            if token == "true" || token == "false" {
                err("Argument cannot be of type boolean!".to_string());
                0
            } else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                } else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                5
            }
        }
    }
}

macro_rules! named {
    ($names:ident, $buffer:ident, $tokens:ident, $next:ident) => {
        if unsafe { NAMED } {
            let ident = $tokens.next().unwrap();
            if $names.contains_key(ident) {
                $buffer.push_u32(*$names.get(ident).unwrap());
            }
            else {
                $names.insert(ident.to_string(), $next);
                $buffer.push_u32($next);
                $next += 1;
            }
        }
        else {
            $buffer.push_u32($tokens.next().unwrap().parse::<u32>().unwrap());
        }
    };
}

pub fn assemble(input: String) -> Vec<u8> {
    let input = remove_quotes(&input);
    let mut buffer = ByteBuffer::new();
    let mut tokens = input.split_whitespace();
    let mut index = 4;
    let mut addresses = Vec::new();
    let mut jumps = Vec::new();
    let mut calls = Vec::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut next_var = 0;
    let mut idents: HashMap<String, u32> = HashMap::new();
    let mut functions: Vec<u32> = Vec::new();
    let mut next_fn = 0;
    let mut returned = true;

    if input.starts_with(".named") {
        unsafe { NAMED = true; }
        tokens.next();
    }

    buffer.push_u32(0);

    while let Some(s) = tokens.next() {
        if s.starts_with(".") {
            if !returned {
                err("There was no return from previous function! This can cause memory errors!".to_string());
            }
            let ident = s.split_at(1).1;
            if !idents.contains_key(ident) {
                idents.insert(ident.to_string(), next_fn);
                next_fn += 1;
            }
            let id = idents[ident];
            if functions.len() <= id as usize {
                functions.resize(id as usize + 1, 0);
            }
            functions[id as usize] = index;
            returned = false;
            continue;
        }
        addresses.push(index);
        index += 1;
        match s.to_ascii_uppercase().as_str() {
            "NOP" => buffer.push_u8(NOOP),
            "END" => {
                buffer.push_u8(END);
                returned = true;
            }
            "MOV" => {
                buffer.push_u8(MOV);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "JMP" => {
                buffer.push_u8(JMP);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JZ" => {
                buffer.push_u8(JZ);
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
                jumps.push(buffer.get_wpos());
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "CMP" => {
                buffer.push_u8(CMP);
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "JE" => {
                buffer.push_u8(JE);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JNE" => {
                buffer.push_u8(JNE);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JG" => {
                buffer.push_u8(JG);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JGE" => {
                buffer.push_u8(JGE);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JL" => {
                buffer.push_u8(JL);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "JLE" => {
                buffer.push_u8(JLE);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap());
                index += 4;
            }
            "CALL" => {
                buffer.push_u8(CALL);
                let call = tokens.next().unwrap();
                if BUILTIN_FUNCTIONS.contains(&call.to_ascii_uppercase().as_str()) {
                    buffer.push_u8(BUILTIN as u8);
                    buffer.push_string(call.to_ascii_uppercase().as_str());
                    index += call.len() as u32 + 5;
                    continue;
                }
                if !idents.contains_key(call) {
                    idents.insert(call.to_string(), next_fn);
                    next_fn += 1;
                }
                let id = idents[call];
                calls.push(buffer.get_wpos());
                buffer.push_u32(id as u32);
                index += 4;
            }
            "RET" => {
                buffer.push_u8(RET);
                returned = true;
            }
            "INC" => {
                buffer.push_u8(INC);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "DEC" => {
                buffer.push_u8(DEC);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "ADD" => {
                buffer.push_u8(ADD);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "SUB" => {
                buffer.push_u8(SUB);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "MUL" => {
                buffer.push_u8(MUL);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "DIV" => {
                buffer.push_u8(DIV);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "MOD" => {
                buffer.push_u8(MOD);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "AND" => {
                buffer.push_u8(AND);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_prim_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "OR" => {
                buffer.push_u8(OR);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_prim_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "NOT" => {
                buffer.push_u8(NOT);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "NEG" => {
                buffer.push_u8(NEG);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "XOR" => {
                buffer.push_u8(XOR);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_prim_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "SHL" => {
                buffer.push_u8(SHL);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "SHR" => {
                buffer.push_u8(SHR);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "SAR" => {
                buffer.push_u8(SAR);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "PUSH" => {
                buffer.push_u8(PUSH);
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "POP" => {
                buffer.push_u8(POP);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "PRINT" => {
                buffer.push_u8(PRINT);
                index += push_str_var(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "SH" => {
                buffer.push_u8(SH);
                index += push_str_var(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "PUSH_RET" => {
                buffer.push_u8(PUSH_RET);
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            "POP_RET" => {
                buffer.push_u8(POP_RET);
                named!(names, buffer, tokens, next_var);
                index += 4;
            }
            "CPY" => {
                buffer.push_u8(CPY);
                named!(names, buffer, tokens, next_var);
                index += 4;
                index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var);
            }
            _ => err(format!("Unknown instruction: {}", s)),
        }
    }

    if !idents.contains_key("main") {
        err("No main function found".to_string());
    }

    buffer.set_wpos(0);
    buffer.push_u32(functions[idents["main"] as usize]);

    for jump in jumps {
        buffer.set_rpos(jump);
        let addr = buffer.pop_u32().unwrap() as usize;
        if addr >= addresses.len() {
            err(format!("Invalid jump address: {}", addr));
        }
        buffer.set_wpos(jump);
        buffer.write_u32(addresses[addr as usize]);
    }

    for call in calls {
        buffer.set_rpos(call);
        let func = buffer.pop_u32().unwrap() as usize;
        if func >= functions.len() {
            err(format!("Invalid call address: {}", func));
        }
        buffer.set_wpos(call);
        buffer.write_u32(functions[func as usize]);
    }

    buffer.into_vec()
}
