use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::{
    error::lexer::{
        make_number::{DoubleDotError, MakeNumberError, InvalidDigitError},
        make_string::{MakeStringError, StringNotClosedError},
        LexerError,
    },
};

pub struct Lexer<'a> {
    pub code: Peekable<Enumerate<Chars<'a>>>,
    pub original: &'a str,
}

impl<'a> Lexer<'a> {
    const IDENTIFIER_TERMINATORS: [char; 5] = [
        ' ',
        ')',
        '(',
        ',',
        '\n'
    ];

    pub fn new(text: &'a str) -> Self {
        Self {
            code: text.chars().enumerate().peekable(),
            original: text,
        }
    }

    fn make_identifier(&mut self, starting_index: usize) -> Result<Token<'a>, LexerError> {
        let mut last_index;

        loop {
            let Some((index, char)) = self.code.peek() else {
                last_index = self.original.len();
                break;
            };

            last_index = *index;

            if Self::IDENTIFIER_TERMINATORS.contains(char) {
                break;
            }

            self.code.next();
            
        }

        let slice = self.original.get(starting_index..last_index).unwrap();
        Ok(
            Token::new(
                TokenType::Identifier(String::from(slice)),
                slice
            ),
        )
    }

    fn make_string(&mut self, starting_index: usize) -> Result<Token<'a>, LexerError> {
        let mut last_index;

        loop {
            let Some((index, char)) = self.code.next() else {
                return Err(
                    LexerError::MakeString(
                        MakeStringError::StringNotClosed(
                            StringNotClosedError::new(starting_index)
                        )
                    )
                );
            };

            last_index = index;

            if char == '"' {
                break;
            };
        }

        let slice = self.original.get(starting_index + 1..last_index).unwrap();
        Ok(Token::new(
            TokenType::Literal(Literal::String(String::from(slice))),
            slice,
        ))
    }

    fn make_number(&mut self, starting_index: usize) -> Result<Token<'a>, LexerError> {

        let mut first_dot: Option<usize> = None;
        let mut prev_index;

        loop {
            let Some((index, char)) = self.code.peek() else {
                prev_index = self.original.len();
                break;
            };

            prev_index = *index;

            if *char == '.' {
                match first_dot {
                    Some(it) => {
                        return Err(LexerError::MakeNumber(MakeNumberError::DoubleDot(
                            DoubleDotError::new(it, *index),
                        )));
                    }
                    None => {
                        first_dot = Some(*index);
                    }
                }

            } else if Self::IDENTIFIER_TERMINATORS.contains(char) {
                break;
            } else if !char.is_digit(10) {
                return Err(LexerError::MakeNumber(MakeNumberError::InvalidDigit(InvalidDigitError::new(*index))))
            }

            self.code.next();
        }

        let str = self.original.get(starting_index..prev_index).unwrap();
        if !first_dot.is_some() {
            match str.parse::<i32>() {
                Ok(it) => Ok(Token::new(TokenType::Literal(Literal::Integer(it)), str)),
                Err(err) => Err(LexerError::MakeNumber(MakeNumberError::ParseInt(err))),
            }
        } else {
            match str.parse::<f32>() {
                Ok(it) => Ok(Token::new(TokenType::Literal(Literal::Float(it)), str)),
                Err(err) => Err(LexerError::MakeNumber(MakeNumberError::ParseFloat(err))),
            }
        }
    }

    fn make_none(&mut self, starting_index: usize) -> Token<'a> {
        pub const IGNORE_CHARS: [char; 3] = [' ', '\n', '\t'];
        let mut prev_index;
        loop {
            let Some((index, next_char)) = self.code.peek() else {
                prev_index = self.original.len();
                break;
            };

            prev_index = *index;

            if !IGNORE_CHARS.contains(next_char) {
                break;
            }

            self.code.next();
        }
        return Token::new(
            TokenType::None,
            self.original.get(starting_index..prev_index).unwrap(),
        );
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((index, char)) = self.code.next() else {
            return None;
        };

        let index = index.clone();

        match char {
            '"' => {
                return Some(self.make_string(index))
            },
            '(' => {
                return Some(Ok(Token::new(TokenType::FunctionStart, self.original.get(index..index + 1).unwrap())));
            },
            ')' => {
                return Some(Ok(Token::new(TokenType::FunctionEnd, self.original.get(index..index + 1).unwrap())));
            },
            ',' => {
                return Some(Ok(Token::new(TokenType::Comma, self.original.get(index..index + 1).unwrap())));
            }
            ';' => {
                return Some(Ok(Token::new(TokenType::Terminator, self.original.get(index..index + 1).unwrap())));
            }

            _ => {}
        }

        if char.is_digit(10) {
            return Some(self.make_number(index));
        }

        if char.is_alphabetic() {
            return Some(self.make_identifier(index))
        }

        return Some(Ok(self.make_none(index)));

    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, slice: &'a str) -> Self {
        Self { token_type, slice }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    None,
    Literal(Literal),
    Identifier(String),
    FunctionStart,
    FunctionEnd,
    Comma,
    Terminator
}


#[derive(Debug, PartialEq)]
pub enum Literal {

    String(String),
    Integer(i32),
    Float(f32)

}