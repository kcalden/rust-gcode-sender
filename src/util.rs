/// Load a file as a string
pub fn load_file(file_path: &str) -> Option<String> {
    let resolved_path = match std::fs::canonicalize(file_path) {
        Ok(path) => path,
        _ => return None
    };
    
    if let Ok(contents) = std::fs::read_to_string(resolved_path) {
        return Some(contents);
    }
    None
}

/// Generate checksum for given string
pub fn checksum(cmd: &str) -> u8 {
    cmd.as_bytes().iter()
        .fold(0u8, |cs, v| cs ^ v)
}