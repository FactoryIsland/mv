pub mod msg;
pub mod script;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use crate::script::assembly::assembler::assemble;
use crate::script::assembly::linker::{AssemblyFile, link};
use crate::script::compiler::codegen::Generator;
use crate::script::compiler::lexer::Lexer;
use crate::script::compiler::parser::Parser;
use crate::script::run::run;

use std::process::exit;

fn main() {
    test_compiler();

    //test_assembler();
}

fn test_compiler() {
    let mut file = OpenOptions::new().read(true).open("mvscript/script.mvs").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let lexer = Lexer::new(code);

    let parser = Parser::new(lexer);

    let result = parser.parse();

    if let Err(e) = result {
        println!("{:?}", e);
        return;
    }
    let result = result.unwrap();

    let generator = Generator::new(result);

    let script = generator.generate();

    let script = AssemblyFile {
        name: "script".to_string(),
        code: script
    };

    let mut file = OpenOptions::new().read(true).open("mvscript/git.mvs").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let lexer = Lexer::new(code);

    let parser = Parser::new(lexer);

    let result = parser.parse();

    if let Err(e) = result {
        println!("{:?}", e);
        return;
    }
    let result = result.unwrap();

    let generator = Generator::new(result);

    let git = generator.generate();

    let git = AssemblyFile {
        name: "git".to_string(),
        code: git
    };

    let linked = link(vec![script, git]);

    let bytecode = assemble(linked);
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("out.mv").unwrap();
    file.write_all(&bytecode).unwrap();
    run(&bytecode, vec![]);
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

    println!("{}", assembly);

    let bytecode = assemble(assembly);
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("masm/script.mv").unwrap();
    file.write_all(&bytecode).unwrap();
    run(&bytecode, vec!["*".to_string(), "Fixed assembler strings".to_string()]);
}