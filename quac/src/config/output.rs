use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PathType {
    File,
    Directory,
}

/// Differentiates whether the given path represents a directory or a file.
///
/// 1. If the path exists on disk and is a directory -> `PathType::Directory`.
/// 2. If the path exists on disk and is a file -> `PathType::File`.
/// 3. If the path does not exist on disk yet:
///    - If it ends with a path separator ('/' or '\\') -> `PathType::Directory`.
///    - Otherwise -> `PathType::File`.
pub fn determine_path_type<P: AsRef<Path>>(path: P) -> PathType {
    let path = path.as_ref();
    if path.is_dir() {
        return PathType::Directory;
    }
    if path.is_file() {
        return PathType::File;
    }

    let path_str = path.to_string_lossy();
    if path_str.ends_with('/')
        || path_str.ends_with('\\')
        || path_str.ends_with(std::path::MAIN_SEPARATOR)
    {
        PathType::Directory
    } else {
        PathType::File
    }
}

/// Creates the directory and any nonexistent parent directories recursively.
///
/// Safe against nested nonexistent directories without panicking.
pub fn create_dir_if_not_exists<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    let path = path.as_ref();
    if !path.is_dir() {
        return fs::create_dir_all(path);
    }

    Ok(())
}

/// Verifies and prepares the output path for the compiler:
///
/// - Differentiates between a directory and a file path.
/// - If `out_path` is a directory (e.g. `.quaoar/`, `build/`):
///     - Creates the directory and all parent directories safely (`create_dir_all`).
///     - Appends the executable name derived from the source file stem (or `"main"` if none).
/// - If `out_path` is a file (e.g. `bin/app`, `app`):
///     - Safely creates parent directories if needed (`create_dir_all`).
///     - Leaves the target file path intact for compiler/linker output without creating
///       it as a directory.
pub fn verify_and_prepare_output<P: AsRef<Path>, S: AsRef<Path>>(
    out_path: P,
    source_path: S,
) -> Result<PathBuf, std::io::Error> {
    let out = out_path.as_ref();
    let source = source_path.as_ref();

    match determine_path_type(out) {
        PathType::Directory => {
            create_dir_if_not_exists(out)?;
            let file_name = source
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("main");
            Ok(out.join(file_name))
        }
        PathType::File => {
            if let Some(parent) = out.parent() {
                if !parent.as_os_str().is_empty() {
                    create_dir_if_not_exists(parent)?;
                }
            }
            Ok(out.to_path_buf())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_path_type() {
        assert_eq!(determine_path_type("foo/bar/"), PathType::Directory);
        assert_eq!(determine_path_type("foo/bar"), PathType::File);
        assert_eq!(determine_path_type("app.exe"), PathType::File);
        assert_eq!(determine_path_type("app"), PathType::File);
        assert_eq!(determine_path_type("."), PathType::Directory);
    }

    #[test]
    fn test_create_dir_nested() {
        let temp_dir = std::env::temp_dir().join("q4r_test_nested_dir/sub1/sub2");
        let res = create_dir_if_not_exists(&temp_dir);
        assert!(res.is_ok());
        assert!(temp_dir.is_dir());
        let _ = fs::remove_dir_all(std::env::temp_dir().join("q4r_test_nested_dir"));
    }

    #[test]
    fn test_verify_and_prepare_output_directory() {
        let temp_dir = std::env::temp_dir().join("q4r_test_out_dir/nested/");
        let source = Path::new("examples/statements.qua");

        let prepared = verify_and_prepare_output(&temp_dir, source).unwrap();
        assert_eq!(prepared, temp_dir.join("statements"));
        assert!(temp_dir.is_dir());

        let _ = fs::remove_dir_all(std::env::temp_dir().join("q4r_test_out_dir"));
    }

    #[test]
    fn test_verify_and_prepare_output_file() {
        let temp_dir = std::env::temp_dir().join("q4r_test_out_file/nested");
        let file_path = temp_dir.join("my_binary");
        let source = Path::new("examples/statements.qua");

        let prepared = verify_and_prepare_output(&file_path, source).unwrap();
        assert_eq!(prepared, file_path);
        assert!(temp_dir.is_dir());
        assert!(!file_path.exists()); // The file itself should NOT be created as a directory

        let _ = fs::remove_dir_all(std::env::temp_dir().join("q4r_test_out_file"));
    }
}
