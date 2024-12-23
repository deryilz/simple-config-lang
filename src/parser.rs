use std::collections::HashMap;

use crate::lexer::Lexer;

pub enum Node {
    Integer(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, Node>),
    Boolean(bool),
    List(Vec<Node>),
    None
}

pub enum ParseError {

}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    fn parse(&mut self) -> ParseResult<Node> {
        // while !lexer.is_done() {
        //     match lexer.next() {
        //         Ok(token) => println!("Token: {:?}", token),
        //         Err(err) => {
        //             println!("Error: {:?}", err);
        //             break;
        //         },
        //     }
        // }
        todo!()
    }


}
