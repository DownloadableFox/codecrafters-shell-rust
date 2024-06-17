use std::{fs, path::PathBuf};

pub enum InternalError {
    ExecutableNotFound,
    UnexpectedState(&'static str)
}

pub fn find_executable(name: &String) -> Result<String, InternalError> {
    if let Ok(path) = std::env::var("PATH") {
        let dirs: Vec<&str> = path.split(':').collect();
        
        for dir in dirs { 
            let files = find_exec_in_dir(dir, name)?;
            
            if let Some(found) = files.first() {
                if let Some(filename) = found.to_str() {
                    return Ok(filename.to_string());
                }
            }
        } 

        Err(InternalError::ExecutableNotFound)
    } else {
        Err(InternalError::UnexpectedState("PATH is not set in env"))
    }
}

pub fn find_exec_in_dir(dir: &str, name: &str) -> Result<Vec<PathBuf>, InternalError> {
    let mut result = Vec::new();

    if let Ok(files) = fs::read_dir(dir) {
        for file in files.flatten() {
            if file.path().is_dir() {
                continue;
            }

            if file.file_name().to_string_lossy() == name {
                result.push(file.path());
            }
        }
    
        Ok(result)
    } else {
        Err(InternalError::UnexpectedState("failed to read directory"))
    }
}
