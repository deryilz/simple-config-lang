use crate::lexer::Lexer;

use std::fs;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Object(Vec<(String, Value)>),
    Boolean(bool),
    List(Vec<Value>),
    None,
}

#[derive(Debug)]
pub enum ParseError {}

pub struct Parser<'a> {
    parens: u8,
    squares: u8,
    curlies: u8,
    chars: &'a [char]
}

impl<'a> Parser<'a> {
    pub fn new(chars: &[char]) -> Parser {
        Parser {
            chars,
            parens: 0,
            curlies: 0,
            squares: 0
        }
    }

    pub fn parse(self) -> Result<Value, ParseError> {
        let lexer = Lexer::new(self.chars);
        for token in lexer {
            println!("{token:?}");
            // match token {}
        }
        todo!()
    }
}
