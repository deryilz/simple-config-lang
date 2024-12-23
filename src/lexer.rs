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
    pub chars: &'a [char],
    i: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &[char]) -> Lexer {
        Lexer { chars, i: 0 }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.i >= self.chars.len() {
            return None;
        }

        let token = self
            .peek_char()
            .or_else(|| self.peek_number())
            .or_else(|| self.peek_word())
            .or_else(|| self.peek_string())
            .or_else(|| self.peek_comment())
            .unwrap_or_else(|| Token {
                kind: TokenKind::Invalid,
                start: self.i,
                end: self.chars.len(),
            });

        Some(token)
    }

    pub fn next(&mut self) -> Option<Token> {
        self.consume_whitespace();

        self.peek().inspect(|token| self.i = token.end)
    }

    fn consume_whitespace(&mut self) {
        while self.i < self.chars.len() {
            match self.chars[self.i] {
                '\n' | ' ' | '\t' => self.i += 1,
                _ => break,
            }
        }
    }

    fn peek_comment(&self) -> Option<Token> {
        let mut i = self.i;

        if self.chars[i] != '#' {
            return None;
        }

        while i < self.chars.len() && self.chars[i] != '\n' {
            i += 1;
        }

        Some(Token {
            kind: TokenKind::Comment,
            start: self.i,
            end: i,
        })
    }

    fn peek_char(&self) -> Option<Token> {
        let kind = match self.chars[self.i] {
            '(' => TokenKind::ParenL,
            ')' => TokenKind::ParenR,
            ']' => TokenKind::SquareL,
            '[' => TokenKind::SquareR,
            '{' => TokenKind::CurlyL,
            '}' => TokenKind::CurlyR,
            ',' => TokenKind::Comma,
            _ => return None,
        };

        Some(Token {
            kind,
            start: self.i,
            end: self.i + 1,
        })
    }

    fn peek_number(&self) -> Option<Token> {
        let mut i = self.i;
        let mut seen_dot = false;
        let mut seen_digit = false;

        while i < self.chars.len() {
            match self.chars[i] {
                '0'..='9' => seen_digit = true,
                '_' => {}
                '-' => {
                    if seen_digit || seen_dot {
                        return None;
                    }
                }
                '.' => {
                    if seen_dot {
                        return None;
                    } else {
                        seen_dot = true;
                    }
                }
                _ => break,
            }
            i += 1;
        }

        if !seen_digit {
            return None;
        }

        let kind = if seen_dot {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };

        Some(Token {
            kind,
            start: self.i,
            end: i,
        })
    }

    fn peek_word(&self) -> Option<Token> {
        let mut i = self.i;

        let kind = match self.chars[i] {
            'a'..='z' => TokenKind::Field,
            'A'..='Z' => TokenKind::Keyword,
            _ => return None,
        };

        i += 1;

        while i < self.chars.len() {
            match self.chars[i] {
                'a'..='z' | 'A'..='Z' | '_' | '0'..'9' => i += 1,
                _ => break,
            }
        }

        Some(Token {
            kind,
            start: self.i,
            end: i,
        })
    }

    // todo: support escaped quotes
    fn peek_string(&self) -> Option<Token> {
        let mut i = self.i;

        if self.chars[i] != '"' {
            return None;
        };

        i += 1;

        while i < self.chars.len() {
            if self.chars[i] == '"' {
                break;
            }
            i += 1;
        }

        // consume the ending quote
        i += 1;

        Some(Token {
            kind: TokenKind::String,
            start: self.i,
            end: i,
        })
    }
}

// weird
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
