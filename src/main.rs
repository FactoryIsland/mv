pub mod msg;
pub mod script;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use crate::script::assembler::assemble;
use crate::script::linker::{AssemblyFile, link};
use crate::script::run::run;

fn main() {
    let mut file = OpenOptions::new().read(true).open("script.masm").unwrap();
    let mut assembly = String::new();
    file.read_to_string(&mut assembly).unwrap();
    let mut file = OpenOptions::new().read(true).open("git.masm").unwrap();
    let mut git = String::new();
    file.read_to_string(&mut git).unwrap();

    let test = AssemblyFile {
        name: "script.masm".to_string(),
        code: assembly
    };

    let lib = AssemblyFile {
        name: "git.masm".to_string(),
        code: git
    };

    let assembly = link(vec![test, lib]);

    let bytecode = assemble(assembly);
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("script.mv").unwrap();
    file.write_all(&bytecode).unwrap();
    run(&bytecode, vec!["*".to_string(), "Fixed assembler strings".to_string()]);
}
