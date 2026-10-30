use std::fs;

use quac_doctor;
use quaoar_c::emitter::CCompiler;
use quac_core::{
    backend::Backend, 
    codegen::Codegen, 
    header::SignatureMounter, 
    lexer::Lexer, 
    linefeeder::feed_lines
};

use crate::{config::conf::Config, helpers};

pub fn compile(conf: Config){
    let source = fs::read(&conf.path);
    let s = match source {
        Ok(_) => source.unwrap(),
        Err(reason) => panic!("{}", reason),
    };
    let path = helpers::get_absolute_path(&conf.path);
    let sourcemap = feed_lines(&s);

    let t = Lexer::new(s.clone()).lexerize();
    if !t.1.is_empty(){
        quac_doctor::diagnose(t.1, s, sourcemap, path);
        return;
    }
    println!("{:#?}", t.0);

    let mut binding = SignatureMounter::new(s.as_slice(), t.0.as_slice());
    let signatures = binding.mount();

    let mut backend = CCompiler::new();
    let mut c_compiler: Backend<'_> = Backend::new(&s, &t.0, signatures, conf.debug_mode);

    let bytes = backend
        .generate_headers(&c_compiler.signatures)
        .generate(&mut c_compiler);

    if !bytes.1.is_empty(){
        quac_doctor::diagnose(c_compiler.errors, s, sourcemap, path);
        return;
    }

    for i in &bytes.0{
        print!("{}", *i as char);
    }
    quaoar_c::exec(&bytes.0, conf.out);
}