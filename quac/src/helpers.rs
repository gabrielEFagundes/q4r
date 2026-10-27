use std::path::PathBuf;

pub fn get_absolute_path(path: &PathBuf) -> PathBuf{
    let path = std::path::absolute(path);
    
    path.expect("rust: error when getting absolute path")
}