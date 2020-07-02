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