use std::{env::args, fs::self};

use quaoar_c::compiler::CCompiler;
use quaoar_core::{backend::Backend, emitter::CodeGen, header::SignatureMounter, lexer::Lexer};

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

    let signatures = SignatureMounter::new(s.as_slice(), t.as_slice()).mount();
    
    // considering I only have the C backend rn
    let backend = CCompiler::new();
    let mut c_compiler: Backend<'_, CCompiler> = Backend::new(
        &s,
        &t,
        signatures,
        backend
    );
    
    let bytes = c_compiler.backend.generate(t.as_slice(), s.as_slice(), &mut c_compiler.cursor);

    for i in &bytes{
        print!("{}", *i as char);
    }
    quaoar_c::exec(&bytes);
}