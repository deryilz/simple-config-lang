use std::iter::Peekable;

use crate::lexer::{Lexer, NoCommentLexer, TokenKind};

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
pub struct ParseError {
    message: String,
    location: usize,
}

pub struct Parser<'a> {
    string: &'a str,
    lexer: Peekable<NoCommentLexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(string: &str) -> Parser {
        Parser {
            string,
            lexer: Lexer::new(string).no_comments().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let value = self.parse_one()?;
        if let Some(token) = self.lexer.peek() {
            return Err(ParseError {
                message: format!("Expected Nothing but got {}", token.kind),
                location: token.start,
            });
        }
        Ok(value)
    }

    fn parse_one(&mut self) -> Result<Value, ParseError> {
        let next = match self.lexer.next() {
            Some(token) => token,
            None => {
                return Err(ParseError {
                    message: "Expected value, got Nothing".to_string(),
                    location: self.string.len(),
                })
            }
        };
        match next.kind {
            TokenKind::SquareL => {
                let values = self.parse_rest_of_list()?;
                Ok(Value::List(values))
            }
            TokenKind::ParenL => {
                let properties = self.parse_rest_of_object()?;
                Ok(Value::Object(properties))
            }
            TokenKind::Float => {
                let str = &self.string[next.start..next.end].replace("_", "");
                match str.parse() {
                    Ok(float) => Ok(Value::Float(float)),
                    Err(_) => Err(ParseError {
                        message: format!("Failed to parse Float {}", str),
                        location: next.start,
                    }),
                }
            }
            TokenKind::Integer => {
                let str = &self.string[next.start..next.end].replace("_", "");
                match str.parse() {
                    Ok(float) => Ok(Value::Integer(float)),
                    Err(_) => Err(ParseError {
                        message: format!("Failed to parse Integer {}", str),
                        location: next.start,
                    }),
                }
            }
            TokenKind::Keyword => {
                let str = &self.string[next.start..next.end];
                match str {
                    "True" => Ok(Value::Boolean(true)),
                    "False" => Ok(Value::Boolean(false)),
                    "None" => Ok(Value::None),
                    other => Err(ParseError {
                        message: format!(
                            "{} is not a valid Keyword. Only None, True, and False are allowed.",
                            other
                        ),
                        location: next.start,
                    }),
                }
            }
            TokenKind::String => {
                let str = &self.string[(next.start + 1)..(next.end - 1)];
                Ok(Value::String(str.to_string()))
            }
            other => Err(ParseError {
                message: format!("Expected value but got {}", other),
                location: next.start,
            }),
        }
    }

    fn parse_rest_of_list(&mut self) -> Result<Vec<Value>, ParseError> {
        let mut values = vec![];
        loop {
            if let Some(token) = self.lexer.peek() {
                if token.kind == TokenKind::SquareR {
                    self.lexer.next();
                    return Ok(values);
                }
            }
            values.push(self.parse_one()?);
            if let Some(token) = self.lexer.next() {
                match token.kind {
                    TokenKind::SquareR => return Ok(values),
                    TokenKind::Comma => {}
                    other_kind => {
                        return Err(ParseError {
                            message: format!("Expected ']' or ',' but got {}", other_kind),
                            location: token.start,
                        });
                    }
                }
            } else {
                return Err(ParseError {
                    message: "Expected ']' or ',' but got Nothing".to_string(),
                    location: self.string.len(),
                });
            }
        }
    }

    fn parse_rest_of_object(&mut self) -> Result<Vec<(String, Value)>, ParseError> {
        let mut properties = vec![];

        loop {
            let (field, location) = if let Some(token) = self.lexer.peek() {
                match &token.kind {
                    TokenKind::ParenR => {
                        self.lexer.next();
                        break;
                    }
                    TokenKind::Field => (&self.string[token.start..token.end], token.start),
                    other_kind => {
                        return Err(ParseError {
                            message: format!("Expected Field but got {}", other_kind),
                            location: token.start,
                        })
                    }
                }
            } else {
                return Err(ParseError {
                    message: "Expected Field but got Nothing".to_string(),
                    location: self.string.len(),
                });
            };

            // consume the field
            self.lexer.next();

            let value = self.parse_one()?;
            properties.push((field, location, value));

            if let Some(token) = self.lexer.next() {
                match token.kind {
                    TokenKind::ParenR => break,
                    TokenKind::Comma => {}
                    other_kind => {
                        return Err(ParseError {
                            message: format!(
                                "Expected ')' or ',' but got {}",
                                other_kind.to_string()
                            ),
                            location: token.start,
                        });
                    }
                }
            } else {
                return Err(ParseError {
                    message: "Expected ')' or ',' but got Nothing".to_string(),
                    location: self.string.len(),
                });
            }
        }

        properties.sort_by_cached_key(|pair| pair.0);

        let mut result = Vec::with_capacity(properties.len());

        let mut prev_field = None;
        for (field, location, value) in properties {
            if !field.chars().all(|v| char::is_lowercase(v) || v == '_') {
                return Err(ParseError {
                    message: format!("Field name {} needs to be snake_case", field),
                    location,
                });
            }

            if prev_field == Some(field) {
                return Err(ParseError {
                    message: format!("Field name {} is a duplicate", field),
                    location,
                });
            }

            prev_field = Some(field);

            result.push((field.to_string(), value));
        }

        Ok(result)
    }
}
