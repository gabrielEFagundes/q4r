use std::fs;

use quac_core::{
    backend::Backend, codegen::Codegen, header::SignatureMounter, lexer::Lexer,
    linefeeder::feed_lines,
};
use quac_doctor;
use quaoar_c::emitter::CCompiler;

use crate::{config::conf::Config, helpers};

pub fn compile(conf: Config) {
    let source = fs::read(&conf.path);
    let s = match source {
        Ok(_) => source.unwrap(),
        Err(reason) => panic!("{}", reason),
    };
    let path = helpers::get_absolute_path(&conf.path);
    let sourcemap = feed_lines(&s);

    let (t, err) = Lexer::new(s.clone()).lexerize();
    if !err.is_empty() {
        quac_doctor::diagnose(err, s, sourcemap, path);
        return;
    }

    let mut binding = SignatureMounter::new(s.as_slice(), t.as_slice());
    let signatures = binding.mount();

    let mut c_compiler = CCompiler::new();
    let mut backend: Backend<'_> = Backend::new(&s, &t, signatures, conf.debug_mode);

    let bytes = c_compiler
        .generate_headers(&backend.signatures)
        .generate(&mut backend);

    if !bytes.1.is_empty() {
        quac_doctor::diagnose(backend.errors, s, sourcemap, path);
        return;
    }

    quaoar_c::exec(&bytes.0, conf.out);
}
