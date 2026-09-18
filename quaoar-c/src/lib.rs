use crate::Compilers::{Gcc, Zig};

pub mod compiler;
pub mod emitter;
pub mod macros;
pub mod r#impl;
pub mod gcc;
pub mod zigcc;

enum Compilers{
    Gcc,
    Zig
}

fn search_compiler() -> Compilers{
    if gcc::exists(){ return Gcc }
    if zigcc::exists(){ return Zig }

    panic!("no C compiler found on PATH\navailable compilers: `gcc`, `zigcc`")
}

pub fn exec(bytes: &[u8]){
    match search_compiler(){
        Compilers::Gcc => gcc::mount_and_exec(bytes),
        Compilers::Zig => zigcc::mount_and_exec(bytes),
    };
}