use std::collections::HashSet;
use mvutils::utils::remove_quotes;
use crate::script::assembler::{extract};

pub struct AssemblyFile {
    pub name: String,
    pub code: String
}

fn err(str: String) {
    eprintln!("{}", str);
    std::process::exit(1);
}


pub fn link(files: Vec<AssemblyFile>) -> String {
    let mut names = files.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
    names.sort_unstable();
    let mut adapted = HashSet::new();
    for name in names.iter() {
        let name = adapt(name.clone());
        if adapted.contains(&name) {
            err(format!("Duplicate adapted file name: \"{}\"!\nMake sure that the files have unique names when excluding special characters ('.', '/', '\\').", name));
        }
        adapted.insert(name);
    }
    drop(adapted);

    files.into_iter().map(|f| {
        let input = remove_quotes(&clean(f.code.trim()));
        println!("{}", input);
        (f.name, input.split_whitespace().collect::<Vec<_>>().join(" "))
    }).enumerate().map(|(i, (name, mut s))| {
        if !s.starts_with(".named") {
            err("Files that are linked are not allowed to be index-accessed. Use '.named' instead.".to_string());
        }

        let (globals, _, externs, labels) = extract(&s);

        for e in externs {
            if names.binary_search(&e).is_err() {
                err(format!("External dependency '{}' not present!", e));
            }
        }

        let name = adapt(name);
        let mut code = String::new();
        if i == 0 {
            code.push_str(".named ");
        }
        let mut tokens = s.split_whitespace();

        while let Some(token) = tokens.next() {
            if token == ".named" {
                continue;
            }
            if token.starts_with('.') {
                let mut ident = token.split_at(1).1;
                if token.ends_with(':') {
                    ident = ident.split_at(ident.len() - 1).0;
                }
                if labels.contains(&ident.to_string()) {
                    code.push_str(&format!(".{}_{}:", name, ident));
                }
                else {
                    code.push_str(token);
                }
                code.push(' ');
                continue;
            }
            if token.starts_with(['$', '&', '*']) {
                let (c, ident) = token.split_at(1);
                if globals.contains(&ident.to_string()) {
                    code.push(c.chars().next().unwrap());
                    code.push_str(&format!("{}_{}", name, ident));
                }
                else {
                    code.push_str(token);
                }
                code.push(' ');
                continue;
            }
            let first = token.chars().next().unwrap();
            if first.is_ascii_alphabetic() || first == '_' {
                if globals.contains(&token.to_string()) {
                    code.push_str(&format!("{}_{}", name, token));
                    code.push(' ');
                    continue;
                }
            }
            code.push_str(token);
            code.push(' ');
            match token.to_ascii_uppercase().as_str() {
                "JMP" | "JE" | "JNE" | "JL" | "JLE" | "JG" | "JGE" => {
                    let to = tokens.next().unwrap();
                    if labels.contains(&to.to_string()) {
                        code.push_str(&format!("{}_{}", name, to));
                    }
                    else {
                        code.push_str(to);
                    }
                    code.push(' ');
                }
                "JZ" | "JNZ" | "JN" | "JNN" => {
                    code.push_str(tokens.next().unwrap());
                    code.push(' ');
                    let to = tokens.next().unwrap();
                    if labels.contains(&to.to_string()) {
                        code.push_str(&format!("{}_{}", name, to));
                    }
                    else {
                        code.push_str(to);
                    }
                    code.push(' ');
                }
                _ => {}
            }
        }

        code
    }).collect()
}

fn clean(s: &str) -> String {
    s.lines().filter_map(|s| {
        let s = s.trim();
        if s.is_empty() || s.starts_with(";") {
            None
        }
        else {
            if s.contains(';') {
                let mut buffer = String::new();
                let mut tokens = s.split(';');
                let mut str = false;
                while let Some(token) = tokens.next() {
                    buffer.push_str(token);
                    if token.ends_with('\'') {
                        buffer.push(';');
                        continue;
                    }
                    if token.chars().filter(|c| *c == '"').count() % 2 == 1 {
                        str = !str;
                    }
                    if str {
                        buffer.push(';');
                        continue;
                    }
                    else {
                        break;
                    }
                }
                buffer.push('\n');
                Some(buffer)
            }
            else {
                let mut s = s.to_string();
                s.push('\n');
                Some(s)
            }
        }
    }).collect()
}

fn adapt(mut s: String) -> String {
    if !s.chars().next().unwrap().is_ascii_alphabetic() {
        s = "a".to_string() + &s;
    }
    s.replace(['.', '/', '\\'], "_")
}