use std::path::PathBuf;

/// Parse CLI arguments and return the target directory path.
/// Defaults to the current directory if no argument is provided.
pub fn get_directory() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    args.get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Validate that `path` exists and is a directory.
/// Returns `Ok(())` on success, or an error message string.
pub fn validate_directory(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("path '{}' does not exist", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }
    Ok(())
}
