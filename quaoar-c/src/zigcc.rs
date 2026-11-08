use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

const TARGET: &str = "-x";
const OBJFILE: &str = "-o";

/// `zig cc -x c -std=c23 -o <prog_name> <<< '<code_bytes>'`
pub fn mount_and_exec(bytes: &[u8], out: PathBuf) -> Option<bool> {
    let fmt_out_path = out.as_os_str().to_str()?;

    let child = Command::new("zigcc")
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
    Command::new("zigcc").arg("--version").status().is_ok()
}

pub fn test_and_exec(bytes: &[u8]) -> bool {
    let mut child = Command::new("zigcc")
        .args([TARGET, "c", "-std=c23", "--fsyntax-only", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let _ = child.stdin.as_ref().unwrap().write_all(bytes);
    let success = child.wait().unwrap().success();

    if success {
        return true;
    }
    false
}
