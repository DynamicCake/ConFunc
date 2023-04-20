pub mod lexer;

pub trait CompilerError {
    fn get_message(&self) -> String;
}