use std::env::args;
use mvutils::args::{ParseArgs, ParsedArgs};

fn main() {
    let args = args().parse();
    if let None = args.command() {
        //print_help();
        return;
    }
    match args.command().unwrap().as_str() {
        "help" => print_help(),
        _ => { //print_help();
        }
    }
}

fn help() {

}
