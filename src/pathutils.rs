use std::path::{Path, PathBuf};

pub fn sanitize(path_buffer: &PathBuf) -> PathBuf {
    let mut sanitized = Vec::new();

    for current in Path::new(path_buffer) {
        let current_string = current
            .to_string_lossy()
            .trim_matches('/')
            .to_string();
        
        match current_string.as_str() {
            ".." => _ = sanitized.pop(),
            "." => _ = 0,
            other => sanitized.push(other.to_string()),
        }
    }

    let mut path = sanitized
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/");
    
    if path.is_empty() {
        path.push_str("/");
    }

    Path::new(&path).to_path_buf()
}
