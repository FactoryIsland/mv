use std::fs::OpenOptions;
use std::io::Read;
use std::process::exit;
use hashbrown::HashSet;
use mvutils::utils::remove_quotes;
use crate::script::assembly::assembler::{extract};
use crate::script::compiler::codegen::Generator;
use crate::script::compiler::lexer::Lexer;
use crate::script::compiler::parser::Parser;

pub struct AssemblyFile {
    pub name: String,
    pub code: String
}

fn err(str: String) -> ! {
    eprintln!("{}", str);
    exit(1);
}


pub fn link(mut files: Vec<AssemblyFile>) -> String {
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

    let externals_needed = files.iter().map(|f| {
        let (_, _, externs, _) = extract(&f.code);
        let mut needed = Vec::new();
        for e in externs {
            if names.binary_search(&e).is_err() {
                needed.push(e);
            }
        }
        needed
    }).flatten().collect::<HashSet<_>>();

    for external in externals_needed {
        const PATHS: [&str; 6] = ["/usr/bin/", "/usr/lib/", "/usr/include/", "/usr/local/bin/", "/usr/local/lib/", "/usr/local/include/"];
        let asm = external.clone() + ".masm";
        let mvs = external.clone() + ".mvs";
        if let Some(mut file) = PATHS.iter().flat_map(|s| {
            let path = s.to_string() + &asm;
            OpenOptions::new().read(true).open(path).ok()
        }).next() {
            let mut code = String::new();
            file.read_to_string(&mut code).expect("Failed to read dependency");

            files.push(AssemblyFile {
                name: external,
                code,
            })
        } else if let Some(mut file) = PATHS.iter().flat_map(|s| {
            let path = s.to_string() + &mvs;
            OpenOptions::new().read(true).open(path).ok()
        }).next() {
            let mut code = String::new();
            file.read_to_string(&mut code).expect("Failed to read dependency");
            let lexer = Lexer::new(code);

            let parser = Parser::new(lexer);

            let result = parser.parse();

            if let Err(e) = result {
                println!("{:?}", e);
                exit(1);
            }
            let result = result.unwrap();

            let generator = Generator::new(result);

            let script = generator.generate();

            files.push(AssemblyFile {
                name: external,
                code: script,
            })
        } else {
            err(format!("External dependency '{}' not present!", external));
        }
    }

    files.into_iter().map(|f| {
        let input = remove_quotes(&clean(f.code.trim()));
        (f.name, input.split_whitespace().collect::<Vec<_>>().join(" "))
    }).enumerate().map(|(i, (name, s))| {
        if !s.starts_with(".named") {
            err("Files that are linked are not allowed to be index-accessed. Use '.named' instead.".to_string());
        }

        let (globals, _, _, labels) = extract(&s);

        let name = adapt(name);
        let mut code = String::new();
        if i == 0 {
            code.push_str(".named ");
        }
        let mut tokens = s.split_whitespace();

        while let Some(mut token) = tokens.next() {
            if token == ".named" {
                continue;
            }
            if token.starts_with('@') {
                let mut ident = token.split_at(1).1;
                if token.ends_with(':') {
                    ident = ident.split_at(ident.len() - 1).0;
                }
                if ident == "static" {
                    code.push_str(&format!("@{}_static:", name));
                    code.push(' ');
                    continue;
                }
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
            if token.starts_with('%') {
                token = token.split_at(1).1;
                code.push('%');
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
            if token == "static" {
                code.push_str(&format!("{}_static", name));
                code.push(' ');
                continue;
            }
            let first = token.chars().next().unwrap();
            if (first.is_ascii_alphabetic() || first == '_') && globals.contains(&token.to_string()) {
                code.push_str(&format!("{}_{}", name, token));
                code.push(' ');
                continue;
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
                    let mut token = tokens.next().unwrap();
                    if token.starts_with('%') {
                        token = token.split_at(1).1;
                        code.push('%');
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
                    }
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
        if s.is_empty() || s.starts_with(';') {
            None
        }
        else if s.contains(';') {
            let mut buffer = String::new();
            let mut str = false;
            for token in s.split(';') {
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
    }).collect()
}

fn adapt(mut s: String) -> String {
    if !s.chars().next().unwrap().is_ascii_alphabetic() {
        s = "a".to_string() + &s;
    }
    s.replace(['.', '/', '\\'], "_")
}