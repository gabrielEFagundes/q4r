use std::{env::args, fs::self};

use quaoar_c::compiler::CCompiler;
use quaoar_core::{generator::CodeGen, lexer::Lexer, signatures::SignatureMounter};

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
    
    let t = Lexer::new(s.clone()).lexerize();

    // SignatureMounter::new(s.as_slice(), t.as_slice()).mount();
    // CCompiler::new(s.as_slice(), t.as_slice()).generate();
}