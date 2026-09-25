use std::{env::args, fs::self};

use quaoar_c::compiler::CCompiler;
use quaoar_core::{backend::Backend, codegen::Codegen, header::SignatureMounter, lexer::Lexer};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2{
        panic!("couldn't find argument for path");
    }

    let is_debug_mode_active = false; // change this here now, `quac` will have a specific flag for this

    let source = fs::read(&args[2]);
    let s = match source{
        Ok(_) => source.unwrap(),
        Err(reason) => panic!("{}", reason)
    };
    
    let t = Lexer::new(s.clone(), is_debug_mode_active).lexerize();
    println!("{:#?}", t);

    let mut binding = SignatureMounter::new(s.as_slice(), t.as_slice());
    let signatures = binding.mount();
    
    // considering I only have the C backend rn
    let mut backend = CCompiler::new();
    let mut c_compiler: Backend<'_> = Backend::new(
        &s,
        &t,
        signatures,
        is_debug_mode_active
    );
    
    let bytes = backend.generate_headers(&c_compiler.signatures).generate(&mut c_compiler);

    for i in &bytes{
        print!("{}", *i as char);
    }
    quaoar_c::exec(&bytes);
}