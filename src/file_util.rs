use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Error as IoError;

#[derive(Debug)]
pub struct FileReadError {
    file_name: String,
    error: IoError
}

impl FileReadError {
    pub fn new(file_name: String, error: IoError) -> FileReadError {
        FileReadError { file_name, error }
    }
}

impl Display for FileReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Error reading file '{}' - {}", self.file_name, self.error))
    }
}

impl Error for FileReadError {}

// TODO: Check
pub fn load_file(input_file: &str) -> Result<String, FileReadError> {
    match fs::read_to_string(input_file) {
        Err(e) => {

            Err(FileReadError::new(input_file.to_string(), e))
        }
        Ok(value) => Ok(value),
    }
}