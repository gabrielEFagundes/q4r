use std::fs;

use quaoar_c::compiler::CCompiler;
use quaoar_core::{
    backend::Backend, 
    codegen::Codegen, 
    header::SignatureMounter, 
    lexer::Lexer
};

use crate::config::conf::Config;

pub fn compile(conf: Config){
    let source = fs::read(conf.path);
    let s = match source {
        Ok(_) => source.unwrap(),
        Err(reason) => panic!("{}", reason),
    };

    let t = Lexer::new(s.clone(), conf.debug_mode).lexerize();

    let mut binding = SignatureMounter::new(s.as_slice(), t.as_slice());
    let signatures = binding.mount();

    // considering I only have the C backend rn
    let mut backend = CCompiler::new();
    let mut c_compiler: Backend<'_> = Backend::new(&s, &t, signatures, conf.debug_mode);

    let bytes = backend
        .generate_headers(&c_compiler.signatures)
        .generate(&mut c_compiler);

    quaoar_c::exec(&bytes);
}