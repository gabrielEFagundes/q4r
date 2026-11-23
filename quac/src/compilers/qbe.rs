use std::fs;

use quac_core::{lexer::Lexer, linefeeder::feed_lines};

use crate::helpers;
use crate::config::conf::Config;

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

    #[allow(unused)]
    let parser = quac_parser::run(&s, &t);
}
