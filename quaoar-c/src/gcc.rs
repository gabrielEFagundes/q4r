use std::{
    io::Write, path::PathBuf, process::{Command, Stdio},
};

const TARGET: &str = "-x";
const OBJFILE: &str = "-o";

/// `gcc -x c -std=c23 -o <prog_name> <<< '<code_bytes>'`
pub fn mount_and_exec(bytes: &[u8], out_path: PathBuf) -> Option<bool> {
    let fmt_out_path = out_path.as_os_str().to_str()?;
    

    let child = Command::new("gcc")
        .args([TARGET, "c", "-std=c23", OBJFILE, fmt_out_path, "-"])
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
