use quac::{
    compilers,
    config::{conf::Config, flags::QuacArgs, helpers, output},
};
use std::env;

fn main() {
    let args = QuacArgs::new(env::args().collect()).init();
    let mut config = Config::default();

    config = args.extract_opts(config);
    helpers::validate_path(&config);
    config.out = match output::verify_and_prepare_output(&config.out, &config.path) {
        Ok(path) => path,
        Err(msg) => panic!("rust: couldn't prepare output path: {}", msg),
    };

    match config.backend {
        quac::config::conf::Backend::QBE => todo!("qbe backend on v0.2!"),
        quac::config::conf::Backend::C => compilers::cc::compile(config),
    }
}
