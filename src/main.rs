pub mod msg;
pub mod fmt;
pub mod script;

use std::env::args;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use bytebuffer::ByteBuffer;
use mvutils::args::{ParseArgs, ParsedArgs};
use mvutils::save::Saver;
use crate::msg::help;
use crate::script::run::run_mvb;

fn main() {
    //let args = args().parse();
    //if let None = args.command() {
    //    help();
    //    return;
    //}
    //match args.command().unwrap().as_str() {
    //    "help" => { /*print_help()*/ }
    //    _ => { /*print_help();*/ }
    //}
    let mut buffer = ByteBuffer::new();
    buffer.push_u8(22);
    buffer.push_u8(25);
    buffer.push_u8('%' as u8);
    buffer.push_u32(0);
    buffer.push_u8(26);
    let bytes = buffer.as_bytes();
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("script.mv").unwrap();
    file.write_all(bytes).unwrap();
    file.flush().unwrap();
    file.sync_all().unwrap();

    let mut file = OpenOptions::new().read(true).open("script.mv").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    run_mvb(&bytes, vec![String::from("Get program arguments")]);
}
