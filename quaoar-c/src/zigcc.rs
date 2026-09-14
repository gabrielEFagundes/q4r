use std::{io::Write, process::{Command, Stdio}};

const TARGET: &str = "-x";
const OBJFILE: &str = "-o";

/// `zig cc -x c -o <prog_name> <<< '<code_bytes>'`
pub fn mount_and_exec(bytes: &[u8]) -> Option<bool>{
    let child = Command::new("zigcc")
        .args([TARGET, "c", OBJFILE, "q4rzig", "-"])
        .stdin(Stdio::piped())
        .spawn().ok()?;
    
    match child.stdin.unwrap().write_all(bytes){
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}