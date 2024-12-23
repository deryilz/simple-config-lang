use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub enum TokenKind {
    ParenL,
    ParenR,
    SquareL,
    SquareR,
    CurlyL,
    CurlyR,
    Integer,
    Float,
    Keyword, // rules, and also True, False, or None
    String,
    Field,
    Comma,
    Pipe,
    Comment,
    Invalid,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    chars: Peekable<CharIndices<'a>>,
    max_index: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            chars: string.char_indices().peekable(),
            max_index: string.len(),
        }
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.consume_whitespace();

        let (start, next) = match self.chars.next() {
            Some(pair) => pair,
            None => return None,
        };

        let kind = match next {
            '(' => Some(TokenKind::ParenL),
            ')' => Some(TokenKind::ParenR),
            '{' => Some(TokenKind::CurlyL),
            '}' => Some(TokenKind::CurlyR),
            '[' => Some(TokenKind::SquareL),
            ']' => Some(TokenKind::SquareR),
            ',' => Some(TokenKind::Comma),
            '|' => Some(TokenKind::Pipe),
            '#' => self.consume_comment(),
            '"' => self.consume_string(),
            '0'..='9' | '-' => self.consume_number(false),
            '.' => self.consume_number(true),
            'a'..='z' => self.consume_field(),
            'A'..='Z' => self.consume_keyword(),
            _ => None,
        };

        if let Some(kind) = kind {
            let end = self.chars.peek().map(|it| it.0).unwrap_or(self.max_index);
            Some(Token { kind, start, end })
        } else {
            self.chars = "".char_indices().peekable(); // no more chars will be returned
            Some(Token {
                kind: TokenKind::Invalid,
                start,
                end: self.max_index,
            })
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some((_, ' ' | '\n' | '\t')) => {}
                _ => break,
            }
            self.chars.next();
        }
    }

    fn consume_comment(&mut self) -> Option<TokenKind> {
        loop {
            match self.chars.peek() {
                Some((_, '\n')) => return Some(TokenKind::Comment),
                None => return None,
                _ => {}
            }
            self.chars.next();
        }
    }

    fn consume_number(&mut self, mut seen_dot: bool) -> Option<TokenKind> {
        loop {
            match self.chars.peek() {
                Some((_, '0'..='9')) => {}
                Some((_, '_')) => {}
                Some((_, '.')) => {
                    if seen_dot {
                        return None;
                    } else {
                        seen_dot = true;
                    }
                }
                Some((_, '-')) => {
                    return None;
                }
                _ => break,
            }
            self.chars.next();
        }

        if seen_dot {
            Some(TokenKind::Float)
        } else {
            Some(TokenKind::Integer)
        }
    }

    fn consume_keyword(&mut self) -> Option<TokenKind> {
        loop {
            match self.chars.peek() {
                Some((_, 'a'..='z' | 'A'..='Z')) => {}
                _ => return Some(TokenKind::Keyword),
            }
            self.chars.next();
        }
    }

    fn consume_field(&mut self) -> Option<TokenKind> {
        loop {
            match self.chars.peek() {
                Some((_, 'a'..='z' | '0'..='9' | '_')) => {}
                _ => return Some(TokenKind::Field),
            }
            self.chars.next();
        }
    }

    // todo: support escaped quotes
    fn consume_string(&mut self) -> Option<TokenKind> {
        loop {
            match self.chars.next() {
                Some((_, '"')) => return Some(TokenKind::String),
                _ => {}
            }
        }
    }
}
