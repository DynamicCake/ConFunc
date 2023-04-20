use self::{make_string::MakeStringError, make_number::MakeNumberError};

pub mod make_identifier;
pub mod make_number;
pub mod make_string;

#[derive(Debug)]
pub enum LexerError {
    MakeString(MakeStringError),
    MakeNumber(MakeNumberError),
}
