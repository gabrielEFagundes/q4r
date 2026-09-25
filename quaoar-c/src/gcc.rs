use std::{io::Write, process::{Command, Stdio}};

const TARGET: &str = "-x";
const OBJFILE: &str = "-o";

/// `gcc -x c -std=c23 -o <prog_name> <<< '<code_bytes>'`
pub fn mount_and_exec(bytes: &[u8]) -> Option<bool>{
    let child = Command::new("gcc")
        .args([TARGET, "c", "-std=c23", OBJFILE, "q4rgcc", "-"])
        .stdin(Stdio::piped())
        .spawn().ok()?;

    let did_compile = match child.stdin.unwrap().write_all(bytes){
        Ok(_) => Some(true),
        Err(_) => Some(false),
    }.unwrap();

    if !did_compile{
        panic!("could not compile with gcc");
    }

    match Command::new("./q4rgcc").spawn(){
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}

pub fn exists() -> bool{
    match Command::new("gcc").arg("--version").status(){
        Ok(_) => true,
        Err(_) => false,
    }
}