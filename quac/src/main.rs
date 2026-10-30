use std::env;
use quac::{
    compilers, config::{
        conf::Config, flags::QuacArgs, helpers, output
    }
};

fn main() {
    let args = QuacArgs::new(env::args().collect()).init();
    let mut config = Config::default();

    config = args.extract_opts(config);
    helpers::validate_path(&config);
    match output::create_dir_if_not_exists(&config.out){
        Ok(_) => {},
        Err(msg) => panic!("rust: couldn't create output dir: {}", msg)
    };

    match config.backend{
        quac::config::conf::Backend::QBE => todo!("qbe backend on v0.2!"),
        quac::config::conf::Backend::C => compilers::cc::compile(config),
    }
}
