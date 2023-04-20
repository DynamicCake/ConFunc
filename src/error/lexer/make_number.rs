use std::{num::{ParseIntError, ParseFloatError}};

use crate::error::CompilerError;


#[derive(Debug)]
pub enum MakeNumberError {
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    InvalidDigit(InvalidDigitError),
    DoubleDot(DoubleDotError)
}

#[derive(Debug)]
pub struct DoubleDotError {
    first_dot: usize,
    second_dot: usize
}

impl CompilerError for DoubleDotError {
    fn get_message(&self) -> String {
        format!("DoubleDotError at char {}, first char at {}", self.second_dot, self.first_dot)
    }
}

impl DoubleDotError {
    pub fn new(first_dot: usize, second_dot: usize) -> Self {
        Self { first_dot, second_dot }
    }
}

#[derive(Debug)]
pub struct InvalidDigitError {
    position: usize
}

impl CompilerError for InvalidDigitError {
    fn get_message(&self) -> String {
        format!("InvalidDecimalLiteralError at char {}", self.position)
    }
}

impl InvalidDigitError {
    pub fn new(pos: usize) -> Self {
        Self { position: pos }
    }
}