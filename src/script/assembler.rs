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
    ($names:ident, $buffer:ident, $token:ident, $next:ident, $func:ident, $globals:ident) => {
        if unsafe { NAMED } {
            let index = $globals.binary_search(&$token.to_string());
            if let Ok(id) = index {
                $buffer.push_u32(id as u32)
            }
            else {
                let t = format!("{}_{}", $func, $token);
                if $names.contains_key(&t) {
                    $buffer.push_u32(*$names.get(&t).unwrap());
                }
                else {
                    $names.insert(t, *$next);
                    $buffer.push_u32(*$next);
                    *$next += 1;
                }
            }
        }
        else {
            $buffer.push_u32($token.parse::<u32>().unwrap());
        }
    };
}

fn push_str_var(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32, func: &str, globals: &[String]) -> u32 {
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
        }
        VARIABLE | REFERENCE | DEREF => {
            named_var!(names, buffer, str, next_var, func, globals);
            offset += 4;
        }
        ARGUMENT => {
            buffer.push_u32(token.parse::<u32>().unwrap());
            offset += 4;
        }
        _ => err(format!("Invalid string identifier: {}", ident))
    }
    offset
}

fn push_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32, func: &str, globals: &[String]) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            buffer.push_u8(LITERAL as u8);
            let str = format_escaped(token.split_at(1).1);
            buffer.push_string(&str);
            str.len() as u32 + 5
        }
        VARIABLE | REFERENCE | DEREF => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var, func, globals);
            5
        }
        ARGUMENT => {
            buffer.push_u8(ARGUMENT as u8);
            buffer.push_u32(token.split_at(1).1.parse::<u32>().unwrap());
            5
        }
        '\'' => {
            let c = parse_char(&token.replace('\'', ""));
            buffer.push_u8(CHAR as u8);
            buffer.push_u32(c as u32);
            5
        }
        _ => {
            if token == "true" {
                buffer.push_u8(BOOLEAN_TRUE as u8);
                1
            }
            else if token == "false" {
                buffer.push_u8(BOOLEAN_FALSE as u8);
                1
            }
            else if token.ends_with(CHAR) {
                buffer.push_u8(CHAR as u8);
                buffer.push_u32(token.split_at(token.len() - 1).0.parse::<u32>().unwrap());
                5
            }
            else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                }
                else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                9
            }
        }
    }
}

fn push_prim_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32, func: &str, globals: &[String]) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            err("Argument cannot be of type string!".to_string());
            0
        }
        VARIABLE | REFERENCE | DEREF => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var, func, globals);
            5
        }
        ARGUMENT => {
            buffer.push_u8(ARGUMENT as u8);
            buffer.push_u32(token.split_at(1).1.parse::<u32>().unwrap());
            5
        }
        '\'' => {
            let c = parse_char(&token.replace('\'', ""));
            buffer.push_u8(CHAR as u8);
            buffer.push_u32(c as u32);
            5
        }
        _ => {
            if token == "true" {
                buffer.push_u8(BOOLEAN_TRUE as u8);
                1
            }
            else if token == "false" {
                buffer.push_u8(BOOLEAN_FALSE as u8);
                1
            }
            else if token.ends_with(CHAR) {
                buffer.push_u8(CHAR as u8);
                buffer.push_u32(token.split_at(token.len() - 1).0.parse::<u32>().unwrap());
                5
            }
            else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                }
                else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                9
            }
        }
    }
}

fn push_num_val(buffer: &mut ByteBuffer, token: &str, names: &mut HashMap<String, u32>, next_var: &mut u32, func: &str, globals: &[String]) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            err("Argument cannot be of type string!".to_string());
            0
        }
        VARIABLE | REFERENCE | DEREF => {
            buffer.push_u8(ident as u8);
            let token = token.split_at(1).1;
            named_var!(names, buffer, token, next_var, func, globals);
            5
        }
        ARGUMENT => {
            buffer.push_u8(ARGUMENT as u8);
            buffer.push_u32(token.split_at(1).1.parse::<u32>().unwrap());
            5
        }
        '\'' => {
            let c = parse_char(&token.replace('\'', ""));
            buffer.push_u8(CHAR as u8);
            buffer.push_u32(c as u32);
            5
        }
        _ => {
            if token == "true" || token == "false" {
                err("Argument cannot be of type boolean!".to_string());
                0
            }
            else if token.ends_with(CHAR) {
                buffer.push_u8(CHAR as u8);
                buffer.push_u32(token.split_at(token.len() - 1).0.parse::<u32>().unwrap());
                5
            }
            else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f64(token.parse::<f64>().unwrap());
                }
                else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i64(token.parse::<i64>().unwrap());
                }
                9
            }
        }
    }
}

macro_rules! named {
    ($names:ident, $buffer:ident, $tokens:ident, $next:ident, $func:ident, $globals:ident) => {
        {
            let mut offset = false;
            let mut token = $tokens.next().unwrap();
            if token.starts_with(VARIABLE) {
                token = token.split_at(1).1;
            }
            if token.starts_with(DEREF) {
                offset = true;
                $buffer.push_u8(DEREF as u8);
                token = token.split_at(1).1;
            }
            if unsafe { NAMED } {
                let index = $globals.binary_search(&token.to_string());
                if let Ok(id) = index {
                    $buffer.push_u32(id as u32)
                }
                else {
                    let ident = format!("{}_{}", $func, token);
                    if $names.contains_key(&ident) {
                        $buffer.push_u32(*$names.get(&ident).unwrap());
                    }
                    else {
                        $names.insert(ident, $next);
                        $buffer.push_u32($next);
                        $next += 1;
                    }
                }
            }
            else {
                $buffer.push_u32(token.parse::<u32>().unwrap());
            }
            if offset {
                5
            }
            else {
                4
            }
        }
    };
}

pub fn jump(token: &str, index: u32, labels: &mut HashMap<String, u32>, calls: &mut Vec<u32>) -> u32 {
    if token.starts_with('-') {
        let location = index - token.split_at(1).1.parse::<u32>().unwrap();
        calls.push(location);
        calls.len() as u32 - 1
    }
    else if token.starts_with('+') {
        let location = index + token.split_at(1).1.parse::<u32>().unwrap();
        calls.push(location);
        calls.len() as u32 - 1
    }
    else if token.chars().next().unwrap().is_digit(10) {
        let location = token.parse::<u32>().unwrap();
        calls.push(location);
        calls.len() as u32 - 1
    }
    else {
        if labels.contains_key(token) {
            *labels.get(token).unwrap()
        }
        else {
            calls.push(0);
            let id = calls.len() as u32 - 1;
            labels.insert(token.to_string(), id);
            id
        }
    }
}

pub fn assemble(input: String) -> Vec<u8> {
    let input = clean(&remove_quotes(&input.trim()));
    let mut globals = extract_globals(&input);
    globals.sort_unstable();
    let mut buffer = ByteBuffer::new();
    let mut tokens = input.split_whitespace();
    let mut index = 8;
    let mut labels = HashMap::new();
    let mut jump_calls = Vec::new();
    let mut addresses = Vec::new();
    let mut jumps = Vec::new();
    let mut calls = Vec::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut next_var = globals.len() as u32;
    let mut func = "".to_string();
    let mut idents: HashMap<String, u32> = HashMap::new();
    let mut functions: Vec<u32> = Vec::new();
    let mut next_fn = 0;
    let mut returned = true;
    let mut table = false;

    if input.starts_with(".named") {
        unsafe { NAMED = true; }
        tokens.next();
    }

    buffer.push_u32(0);
    buffer.push_u32(0);

    macro_rules! push_val {
        () => {
            index += push_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var, &func, &globals);
        };
    }

    macro_rules! push_num {
        () => {
            index += push_num_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var, &func, &globals);
        };
    }

    macro_rules! push_prim {
        () => {
            index += push_prim_val(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var, &func, &globals);
        };
    }

    macro_rules! push_str {
        () => {
            index += push_str_var(&mut buffer, tokens.next().unwrap(), &mut names, &mut next_var, &func, &globals);
        };
    }

    macro_rules! get_named {
        () => {
            index += named!(names, buffer, tokens, next_var, func, globals);
        }
    }

    macro_rules! jmp {
        () => {
            let token = tokens.next().unwrap();
            if token.starts_with(VARIABLE) {
                table = true;
                let token = token.split_at(1).1;
                buffer.push_u8(VARIABLE as u8);
                if unsafe { NAMED } {
                    let index = globals.binary_search(&token.to_string());
                    if let Ok(id) = index {
                        buffer.push_u32(id as u32)
                    }
                    else {
                        let ident = format!("{}_{}", func, token);
                        if names.contains_key(&ident) {
                            buffer.push_u32(*names.get(&ident).unwrap());
                        }
                        else {
                            names.insert(ident, next_var);
                            buffer.push_u32(next_var);
                            next_var += 1;
                        }
                    }
                }
                else {
                    buffer.push_u32(token.parse::<u32>().unwrap());
                }
                index += 5;
            }
            else {
                jumps.push(buffer.get_wpos());
                buffer.push_u32(jump(token, addresses.len() as u32 - 1, &mut labels, &mut jump_calls));
                index += 4;
            }
        };
    }

    while let Some(s) = tokens.next() {
        if s == ".global" {
            tokens.next();
            continue;
        }
        else if s.starts_with("@") {
            if !returned {
                err("There was no return from previous function! This can lead to undefined behaviour!".to_string());
            }
            let mut ident = s.split_at(1).1;
            if ident.ends_with(':') {
                ident = ident.split_at(ident.len() - 1).0;
            }
            if !idents.contains_key(ident) {
                idents.insert(ident.to_string(), next_fn);
                next_fn += 1;
            }
            let id = idents[ident];
            if functions.len() <= id as usize {
                functions.resize(id as usize + 1, 0);
            }
            func = ident.to_string();
            functions[id as usize] = index;
            returned = false;
            continue;
        }
        else if s.starts_with(".") {
            let mut ident = s.split_at(1).1;
            if ident.ends_with(':') {
                ident = ident.split_at(ident.len() - 1).0;
            }
            if !labels.contains_key(ident) {
                jump_calls.push(addresses.len() as u32);
                labels.insert(ident.to_string(), jump_calls.len() as u32 - 1);
            }
            else {
                jump_calls[labels[ident] as usize] = addresses.len() as u32;
            }
            continue;
        }
        if func.is_empty() {
            println!("Found instructions outside a function! These will not be called unless jumped to!");
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
                get_named!();
                push_val!();
            }
            "JMP" => {
                buffer.push_u8(JMP);
                jmp!();
            }
            "JZ" => {
                buffer.push_u8(JZ);
                push_val!();
                jmp!();
            }
            "JNZ" => {
                buffer.push_u8(JNZ);
                push_val!();
                jmp!();
            }
            "JN" => {
                buffer.push_u8(JN);
                push_val!();
                jmp!();
            }
            "JNN" => {
                buffer.push_u8(JNN);
                push_val!();
                jmp!();
            }
            "CMP" => {
                buffer.push_u8(CMP);
                push_val!();
                push_val!();
            }
            "JE" => {
                buffer.push_u8(JE);
                jmp!();
            }
            "JNE" => {
                buffer.push_u8(JNE);
                jmp!();
            }
            "JG" => {
                buffer.push_u8(JG);
                jmp!();
            }
            "JGE" => {
                buffer.push_u8(JGE);
                jmp!();
            }
            "JL" => {
                buffer.push_u8(JL);
                jmp!();
            }
            "JLE" => {
                buffer.push_u8(JLE);
                jmp!();
            }
            "CALL" => {
                buffer.push_u8(CALL);
                let call = tokens.next().unwrap();
                if BUILTIN_FUNCTIONS.contains_key(&call.to_ascii_uppercase().as_str()) {
                    buffer.push_u8(BUILTIN as u8);
                    buffer.push_u32(*BUILTIN_FUNCTIONS.get(call.to_ascii_uppercase().as_str()).unwrap());
                    index += 5;
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
                get_named!();
            }
            "DEC" => {
                buffer.push_u8(DEC);
                get_named!();
            }
            "ADD" => {
                buffer.push_u8(ADD);
                get_named!();
                push_num!();
            }
            "SUB" => {
                buffer.push_u8(SUB);
                get_named!();
                push_num!();
            }
            "MUL" => {
                buffer.push_u8(MUL);
                get_named!();
                push_num!();
            }
            "DIV" => {
                buffer.push_u8(DIV);
                get_named!();
                push_num!();
            }
            "MOD" => {
                buffer.push_u8(MOD);
                get_named!();
                push_num!();
            }
            "AND" => {
                buffer.push_u8(AND);
                get_named!();
                push_prim!();
            }
            "OR" => {
                buffer.push_u8(OR);
                get_named!();
                push_prim!();
            }
            "NOT" => {
                buffer.push_u8(NOT);
                get_named!();
            }
            "NEG" => {
                buffer.push_u8(NEG);
                get_named!();
            }
            "XOR" => {
                buffer.push_u8(XOR);
                get_named!();
                push_prim!();
            }
            "SHL" => {
                buffer.push_u8(SHL);
                get_named!();
                push_num!();
            }
            "SHR" => {
                buffer.push_u8(SHR);
                get_named!();
                push_num!();
            }
            "SAR" => {
                buffer.push_u8(SAR);
                get_named!();
                push_num!();
            }
            "PUSH" => {
                buffer.push_u8(PUSH);
                push_val!();
            }
            "POP" => {
                buffer.push_u8(POP);
                get_named!();
            }
            "PRINT" => {
                buffer.push_u8(PRINT);
                push_str!();
            }
            "SH" => {
                buffer.push_u8(SH);
                push_str!();
            }
            "PUSH_RET" => {
                buffer.push_u8(PUSH_RET);
                push_val!();
            }
            "POP_RET" => {
                buffer.push_u8(POP_RET);
                get_named!();
            }
            "CPY" => {
                buffer.push_u8(CPY);
                get_named!();
                push_val!();
            }
            _ => err(format!("Unknown instruction: {}", s)),
        }
    }

    if !idents.contains_key("main") {
        err("No main function found".to_string());
    }

    buffer.set_wpos(0);
    buffer.push_u32(functions[idents["main"] as usize]);
    buffer.push_u32(buffer.len() as u32);
    if table {
        buffer.set_wpos(buffer.len());
        for addr in addresses.iter() {
            buffer.push_u32(*addr);
        }
    }

    for jump in jumps {
        buffer.set_rpos(jump);
        let addr = buffer.pop_u32().unwrap() as usize;
        if addr >= jump_calls.len() {
            err(format!("Invalid jump address: {}", addr));
        }
        buffer.set_wpos(jump);
        buffer.write_u32(addresses[jump_calls[addr as usize] as usize]);
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

fn clean(s: &str) -> String {
    s.lines().filter_map(|s| {
        let s = s.trim();
        if s.is_empty() || s.starts_with(";") {
            None
        }
        else {
            let mut s = s.split(';').next().unwrap().to_string();
            s.push('\n');
            Some(s)
        }
    }).collect()
}

fn extract_globals(s: &str) -> Vec<String> {
    s.lines().filter_map(|s| {
        let mut tokens = s.trim().split_whitespace();
        if tokens.next().unwrap() == ".global" {
            Some(tokens.next().unwrap().to_string())
        }
        else {
            None
        }
    }).collect()
}

fn parse_char(s: &str) -> char {
    if s.len() == 1 {
        s.chars().next().unwrap()
    }
    else if s.len() == 2 && s.chars().next().unwrap() == '\\' {
        let c = s.chars().nth(1).unwrap();
        match c {
            's' => ' ',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'b' => '\x08',
            'f' => '\x0c',
            'v' => '\x0b',
            _ => c
        }
    }
    else if s.starts_with("\\x") || s.starts_with("\\u") {
        let c = u32::from_str_radix(&s[2..], 16).unwrap();
        let c = char::from_u32(c);
        if let None = c {
            err(format!("Invalid hex character: {}", s));
        }
        c.unwrap()
    }
    else {
        err(format!("Invalid string literal for character: {}", s));
        '\0'
    }
}