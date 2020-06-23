pub fn load_file(file_path: &str) -> Option<String> {
    if let Ok(contents) = std::fs::read_to_string(file_path) {
        return Some(contents);
    }
    None
}