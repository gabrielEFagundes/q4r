use std::{fs, path::PathBuf};

pub fn create_dir_if_not_exists(path: &PathBuf) -> Result<(), std::io::Error>{
    if !path.is_dir(){
        return fs::create_dir(path)
    }

    Ok(())
}