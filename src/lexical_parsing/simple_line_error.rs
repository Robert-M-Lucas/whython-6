use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::error::BoxedError;


#[derive(Debug)]
pub struct SimpleLineError {
    description: String,
    line_index: usize,
    file_name: String
}

impl SimpleLineError {
    pub fn new(description: String, line_index: usize, file_name: String) -> SimpleLineError {
        SimpleLineError { description, line_index, file_name }
    }
}

impl Display for SimpleLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} - Line {}: {}", self.file_name, self.line_index + 1, self.description))
    }
}

impl Error for SimpleLineError {}
