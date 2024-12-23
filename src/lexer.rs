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
    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    value: TokenValue,
    start: usize,
    end: usize,
}

pub type LexerResult<T> = Result<T, ()>;

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
        if self.is_done() {
            return Ok(Token {
                value: TokenValue::EndOfFile,
                start: self.current_char,
                end: self.current_char,
            });
        }

        self.peek_char()
            .or_else(|_| self.peek_keyword())
            .or_else(|_| self.peek_number())
            .or_else(|_| self.peek_field())
            .or_else(|_| self.peek_string())
    }

    pub fn next(&mut self) -> LexerResult<Token> {
        self.consume_whitespace();

        let token = self.peek()?;
        self.current_char = token.end;
        Ok(token)
    }

    fn consume_whitespace(&mut self) {
        while !self.is_done() {
            match self.c() {
                '\n' | ' ' | '\r' | '\t' => self.current_char += 1,
                '#' => self.consume_comment(),
                _ => break
            }
        }
    }

    fn consume_comment(&mut self) {
        while !self.is_done() && self.c() != '\n' {
            self.current_char += 1;
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
            _ => return Err(()),
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
                '_' => {}
                '-' => {
                    if seen_digit || seen_dot {
                        return Err(());
                    }
                }
                '.' => {
                    if seen_dot {
                        return Err(());
                    } else {
                        seen_dot = true;
                    }
                }
                _ => break,
            }
            c += 1;
        }

        if !seen_digit {
            return Err(());
        }

        let string: String = self.chars[start..c]
            .iter()
            .copied()
            .filter(|c| c != &'_')
            .collect();

        let value = if seen_dot {
            let f64 = string.parse().map_err(|_| ())?;
            TokenValue::Float(f64)
        } else {
            let i64 = string.parse().map_err(|_| ())?;
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

        Err(())
    }

    fn peek_field(&self) -> LexerResult<Token> {
        let start = self.current_char;

        match self.chars[start] {
            'a'..='z' => {}
            _ => return Err(()),
        };

        let mut c = start + 1;
        while c < self.chars.len() {
            match self.chars[c] {
                'a'..='z' | '_' => c += 1,
                ' ' => break,
                _ => return Err(()),
            }
        }

        Ok(Token {
            value: TokenValue::Field,
            start,
            end: c,
        })
    }

    // todo: support escaped quotes
    fn peek_string(&self) -> LexerResult<Token> {
        let start = self.current_char;

        if self.chars[start] != '"' {
            return Err(());
        };

        let mut c = start + 1;
        while c < self.chars.len() && self.chars[c] != '"' {
            c += 1;
        }

        let string: String = self.chars[(start + 1)..c].iter().collect();

        Ok(Token {
            value: TokenValue::String(string),
            start,
            end: c + 1,
        })
    }

    fn c(&self) -> char {
        self.chars[self.current_char]
    }

    pub fn is_done(&self) -> bool {
        self.current_char >= self.chars.len()
    }
}
