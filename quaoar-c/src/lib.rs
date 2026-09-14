use std::process::Command;

pub mod compiler;
pub mod genhelper;
pub mod cmacros;
pub mod impls;
pub mod gcc;
pub mod zigcc;

enum Compilers{
    Gcc,
    Zig
}

fn search_compiler() -> Compilers{
    match Command::new("zigcc").arg("--version").status(){
        Ok(_) => Compilers::Zig,
        Err(_) => Compilers::Gcc,
    }
}

pub fn exec(bytes: &[u8]){
    match search_compiler(){
        Compilers::Gcc => gcc::mount_and_exec(bytes),
        Compilers::Zig => zigcc::mount_and_exec(bytes),
    };
}