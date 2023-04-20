use crate::error::CompilerError;

#[derive(Debug)]
pub enum MakeStringError {
    StringNotClosed(StringNotClosedError)
}

#[derive(Debug)]
pub struct StringNotClosedError {
    first_position: usize
}

impl CompilerError for StringNotClosedError {
    fn get_message(&self) -> String {
        format!("StringNotClosedError at char {}", self.first_position)
    }
}

impl StringNotClosedError {
    pub fn new(pos: usize) -> Self {
        Self { first_position: pos }
    }
}