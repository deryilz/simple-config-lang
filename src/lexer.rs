use std::{fs, path::Path};

#[derive(Debug)]
pub enum TokenValue {
    ParenL,
    ParenR,
    SquareL,
    SquareR,
    CurlyL,
    CurlyR,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    RuleName,
    String(String),
    None,
    Field,
    Comma,
}

#[derive(Debug)]
pub struct Token {
    value: TokenValue,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken,
    InvalidNumber,
    UnclosedComment,
}

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,
    current_char: usize,
}

impl Lexer {
    pub fn from_file(file: &Path) -> Option<Self> {
        fs::read_to_string(file).ok().map(|string| Lexer {
            chars: string.chars().collect(),
            current_char: 0,
        })
    }

    pub fn from_string(string: &str) -> Self {
        Lexer {
            chars: string.chars().collect(),
            current_char: 0,
        }
    }

    pub fn peek(&self) -> LexerResult<Token> {
        todo!()
    }

    pub fn next(&mut self) -> LexerResult<Token> {
        
        todo!()
    }

    fn consume_whitespace(&mut self) {
        while !self.is_done() {
            let c = self.c();

            if c != '\n' && c != ' ' && c != '\r' && c != '\t' {
                break;
            }

            if c == '#' {
                self.consume_comment();
            } else {
                self.current_char += 1;
            }
        }
    }

    fn consume_comment(&mut self) {
        while !self.is_done() {
            self.current_char += 1;
            if self.c() == '\n' {
                break;
            }
        }
    }

    fn peek_char(&self) -> LexerResult<Token> {
        let value = match self.c() {
            '(' => TokenValue::ParenL,
            ')' => TokenValue::ParenR,
            ']' => TokenValue::SquareL,
            '[' => TokenValue::SquareR,
            '{' => TokenValue::CurlyL,
            '}' => TokenValue::CurlyR,
            ',' => TokenValue::Comma,
            _ => return Err(LexerError::InvalidToken),
        };

        Ok(Token {
            value,
            start: self.current_char,
            end: self.current_char + 1,
        })
    }

    fn peek_number(&self) -> LexerResult<Token> {
        let start = self.current_char;

        let mut c = start;
        let mut seen_dot = false;
        let mut seen_digit = false;

        while c < self.chars.len() {
            match self.chars[c] {
                '0'..='9' => seen_digit = true,
                '-' => {
                    if seen_digit || seen_dot {
                        return Err(LexerError::InvalidNumber);
                    }
                }
                '.' => {
                    if seen_dot {
                        return Err(LexerError::InvalidNumber);
                    } else {
                        seen_dot = true;
                    }
                }
                _ => break,
            }
            c += 1;
        }

        if !seen_digit {
            return Err(LexerError::InvalidNumber);
        }

        let string: String = self.chars[start..c].iter().collect();
        let value = if seen_dot {
            let f64 = string.parse().map_err(|_| LexerError::InvalidNumber)?;
            TokenValue::Float(f64)
        } else {
            let i64 = string.parse().map_err(|_| LexerError::InvalidNumber)?;
            TokenValue::Integer(i64)
        };

        Ok(Token {
            value,
            start,
            end: c,
        })
    }

    fn peek_keyword(&self) -> LexerResult<Token> {
        let start = self.current_char;
        let rem = self.chars.len() - start;
        let keywords = [
            ("True", TokenValue::Boolean(true)),
            ("False", TokenValue::Boolean(false)),
            ("None", TokenValue::None),
        ];

        for (keyword, value) in keywords {
            let len = keyword.len();
            let chars: Vec<char> = keyword.chars().collect();
            let end = start + len;
            if rem >= len && self.chars[start..end] == chars {
                return Ok(Token { value, start, end });
            }
        }

        Err(LexerError::InvalidToken)
    }

    fn c(&self) -> char {
        self.chars[self.current_char]
    }

    fn is_done(&self) -> bool {
        self.current_char >= self.chars.len()
    }
}
