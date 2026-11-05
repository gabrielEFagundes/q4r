use std::{
    io::Write, 
    path::PathBuf,
    process::{Command, Stdio},
};

const TARGET: &str = "-x";
const OBJFILE: &str = "-o";

/// `gcc -x c -std=c23 -o <prog_name> <<< '<code_bytes>'`
pub fn mount_and_exec(bytes: &[u8], out_path: PathBuf, main_name: &str) -> Option<bool> {
    let fmt_out_path = format!("{}/{}", out_path.as_os_str().to_str()?, main_name);

    let child = Command::new("gcc")
        .args([TARGET, "c", "-std=c23", OBJFILE, &fmt_out_path, "-"])
        .stdin(Stdio::piped())
        .spawn()
        .ok()?;

    match child.stdin.unwrap().write_all(bytes) {
        Ok(_) => Some(true),
        Err(_) => Some(false),
    }
}

pub fn exists() -> bool {
    Command::new("gcc").arg("--version").status().is_ok()
}

pub fn test_and_exec(bytes: &[u8]) -> bool{
    let mut child = Command::new("gcc")
        .args([TARGET, "c", "-std=c23", "-fsyntax-only", "-"])
        .stdin(Stdio::piped())
        .spawn().unwrap();

    let _ = child.stdin.as_ref().unwrap().write_all(bytes);
    let success = child.wait().unwrap().success();
    
    if success{ return true }
    false
}