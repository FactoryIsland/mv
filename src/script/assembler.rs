use bytebuffer::ByteBuffer;
use mvutils::save::{Loader, Saver};
use mvutils::utils::{remove_quotes, format_escaped};
use crate::script::consts::*;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

fn push_str_var(buffer: &mut ByteBuffer, token: &str) -> u32 {
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
            buffer.push_u32(str.parse::<u32>().unwrap());
            offset += 4;
        },
        _ => err(format!("Invalid string identifier: {}", ident))
    }
    offset
}

fn push_val(buffer: &mut ByteBuffer, token: &str) -> u32 {
    let ident = token.chars().next().unwrap();
    match ident {
        LITERAL => {
            buffer.push_u8(LITERAL as u8);
            let str = format_escaped(token.chars().skip(1).collect::<String>().as_str());
            buffer.push_string(&str);
            str.len() as u32 + 5
        }
        VARIABLE | ARGUMENT => {
            buffer.push_u8(ident as u8);
            buffer.push_u32(token.parse::<u32>().unwrap());
            5
        }
        _ => {
            if token == "true" {
                buffer.push_u8(BOOLEAN as u8);
                buffer.push_u8(1);
                2
            }
            else if token == "false" {
                buffer.push_u8(BOOLEAN as u8);
                buffer.push_u8(0);
                2
            }
            else {
                if token.contains('.') {
                    buffer.push_u8(FLOAT as u8);
                    buffer.push_f32(token.parse::<f32>().unwrap());
                }
                else {
                    buffer.push_u8(INTEGER as u8);
                    buffer.push_i32(token.parse::<i32>().unwrap());
                }
                5
            }
        }
    }
}

pub fn assemble(input: String) -> Vec<u8> {
    let input = remove_quotes(&input);
    let mut buffer = ByteBuffer::new();
    let mut tokens = input.split_whitespace();
    let mut index = 0;
    let mut addresses = Vec::new();
    let mut jumps = Vec::new();

    while let Some(s) = tokens.next() {
        addresses.push(index);
        index += 1;
        match s {
            "NOP" => buffer.push_u8(NOOP),
            "END" => buffer.push_u8(END),
            "MOV" => {
                buffer.push_u8(MOV);
                let ident = tokens.next().unwrap().parse::<u32>().unwrap();
                buffer.push_u32(ident);
                index += 4;
                index += push_val(&mut buffer, tokens.next().unwrap());
            }
            "JMP" => {
                buffer.push_u8(JMP);
                jumps.push(buffer.get_wpos());
                buffer.push_u32(tokens.next().unwrap().parse::<u32>().unwrap() - 1);
                index += 4;
            }
            "PRINT" => {
                buffer.push_u8(PRINT);
                index += push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "SH" => {
                buffer.push_u8(SH);
                index += push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_ADD_ALL" => buffer.push_u8(GIT_ADD_ALL),
            "GIT_ADD" => {
                buffer.push_u8(GIT_ADD);
                index += push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_COMMIT_DEFAULT" => buffer.push_u8(GIT_COMMIT_DEFAULT),
            "GIT_COMMIT" => {
                buffer.push_u8(GIT_COMMIT);
                index += push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_PUSH_UPSTREAM" => buffer.push_u8(GIT_PUSH_UPSTREAM),
            "GIT_PUSH" => {
                buffer.push_u8(GIT_PUSH);
                index += push_str_var(&mut buffer, tokens.next().unwrap());
            }
            _ => err(format!("Unknown instruction: {}", s)),
        }
    }

    for jump in jumps {
        buffer.set_rpos(jump);
        let addr = buffer.pop_u32().unwrap() as usize;
        if addr >= addresses.len() {
            err(format!("Invalid jump address: {}", addr));
        }
        buffer.set_wpos(jump);
        buffer.write_u32(addresses[addr as usize]);
    }

    buffer.into_vec()
}
