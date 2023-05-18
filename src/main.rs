mod msg;
mod fmt;
mod script;

use std::env::args;
use mvutils::args::{ParseArgs, ParsedArgs};
use crate::msg::help;

fn main() {
    let args = args().parse();
    if let None = args.command() {
        help();
        return;
    }
    match args.command().unwrap().as_str() {
        "help" => { /*print_help()*/ }
        _ => { /*print_help();*/ }
    }
}
