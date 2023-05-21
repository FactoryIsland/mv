pub mod msg;
pub mod script;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use crate::script::assembly::assembler::assemble;
use crate::script::assembly::linker::{AssemblyFile, link};
use crate::script::compiler::lexer::Lexer;
use crate::script::run::run;

fn main() {
    test_compiler();

    //test_assembler();
}

fn test_compiler() {
    let mut file = OpenOptions::new().read(true).open("mvscript/script.mvs").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let lexer = Lexer::new(code);

    for token in lexer {
        println!("{:?}", token);
    }
}

fn test_assembler() {
    let mut file = OpenOptions::new().read(true).open("masm/test.masm").unwrap();
    let mut assembly = String::new();
    file.read_to_string(&mut assembly).unwrap();
    let mut file = OpenOptions::new().read(true).open("masm/git.masm").unwrap();
    let mut git = String::new();
    file.read_to_string(&mut git).unwrap();

    let test = AssemblyFile {
        name: "test.masm".to_string(),
        code: assembly
    };

    let lib = AssemblyFile {
        name: "git.masm".to_string(),
        code: git
    };

    let assembly = link(vec![test]);

    let bytecode = assemble(assembly);
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("masm/script.mv").unwrap();
    file.write_all(&bytecode).unwrap();
    run(&bytecode, vec!["*".to_string(), "Fixed assembler strings".to_string()]);
}