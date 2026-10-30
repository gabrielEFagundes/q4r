use std::path::PathBuf;

use quac_core::tokens::VoidstarTokenTypes;

use crate::Compilers::{Gcc, Zig};

pub mod emitter;
pub mod compiler;
pub mod gcc;
pub mod r#impl;
pub mod macros;
pub mod zigcc;

const TERMINATORS: [VoidstarTokenTypes; 2] = [
    VoidstarTokenTypes::SemiColon,
    VoidstarTokenTypes::CloseBraces,
];

enum Compilers {
    Gcc,
    Zig,
}

fn search_compiler() -> Compilers {
    if gcc::exists() {
        return Gcc;
    }
    if zigcc::exists() {
        return Zig;
    }

    panic!("no C compiler found on PATH\navailable compilers: `gcc`, `zigcc`")
}

pub fn exec(bytes: &[u8], out_path: PathBuf) {
    match search_compiler() {
        Compilers::Gcc => gcc::mount_and_exec(bytes, out_path),
        Compilers::Zig => zigcc::mount_and_exec(bytes, out_path),
    };
}
