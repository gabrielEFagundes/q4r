use std::{env::args, fs::self};

use quaoar_core::lexer::Lexer;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2{
        panic!("couldn't find argument for path");
    }

    let source = fs::read(&args[2]);
    let s = match source{
        Ok(_) => source.unwrap(),
        Err(reason) => panic!("{}", reason)
    };
    
    Lexer::new(s).lexerize();
}