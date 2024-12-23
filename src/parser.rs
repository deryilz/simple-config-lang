use crate::lexer::Lexer;

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
    string: &'a str
}

impl<'a> Parser<'a> {
    pub fn new(string: &str) -> Parser {
        Parser {
            string,
            parens: 0,
            curlies: 0,
            squares: 0
        }
    }

    pub fn parse(self) -> Result<Value, ParseError> {
        let lexer = Lexer::new(self.string);
        for token in lexer {
            println!("{token:?}");
            println!("Content: {}", &self.string[token.start..token.end]);
            // match token {}
        }
        todo!()
    }
}
