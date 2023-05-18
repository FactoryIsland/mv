use bytebuffer::ByteBuffer;
use mvutils::save::Saver;
use mvutils::utils::remove_quotes;
use crate::script::consts::*;

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}

fn push_str_var(buffer: &mut ByteBuffer, token: &str) {
    let mut chars = token.chars();
    let ident = chars.next().unwrap();
    let str = chars.as_str();
    buffer.push_u8(ident as u8);
    match ident {
        LITERAL => buffer.push_string(&str.replace("\\s", " ")),
        VARIABLE | ARGUMENT => buffer.push_u32(str.parse::<u32>().unwrap()),
        _ => err(format!("Invalid string identifier: {}", ident))
    }
}

pub fn assemble(input: String) -> Vec<u8> {
    let input = remove_quotes(&input);
    let mut buffer = ByteBuffer::new();
    let mut tokens = input.split_whitespace();

    while let Some(s) = tokens.next() {
        match s {
            "NOP" => buffer.push_u8(NOOP),
            "PRINT" => {
                buffer.push_u8(PRINT);
                push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "SH" => {
                buffer.push_u8(SH);
                push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_ADD_ALL" => buffer.push_u8(GIT_ADD_ALL),
            "GIT_ADD" => {
                buffer.push_u8(GIT_ADD);
                push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_COMMIT_DEFAULT" => buffer.push_u8(GIT_COMMIT_DEFAULT),
            "GIT_COMMIT" => {
                buffer.push_u8(GIT_COMMIT);
                push_str_var(&mut buffer, tokens.next().unwrap());
            }
            "GIT_PUSH_UPSTREAM" => buffer.push_u8(GIT_PUSH_UPSTREAM),
            "GIT_PUSH" => {
                buffer.push_u8(GIT_PUSH);
                push_str_var(&mut buffer, tokens.next().unwrap());
            }
            _ => err(format!("Unknown instruction: {}", s)),
        }
    }

    buffer.into_vec()
}