use std::path::{Path, PathBuf};

pub fn get_absolute_path(path: &PathBuf) -> PathBuf {
    let path = std::path::absolute(path);

    path.expect("rust: internal error when getting absolute path")
}

pub fn get_file_name(path: &Path) -> &str {
    path.file_stem()
        .expect("rust: internal error when getting file name")
        .to_str()
        .expect("rust: internal error when converting from os_str to str")
}
