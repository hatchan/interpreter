use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        let b = input.as_bytes();
        let mut lexer = Self {
            input: b,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut skip_read = false;
        let token = match self.ch {
            None => Token::new(TokenType::EOF),
            Some(b'=') => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Token::new(TokenType::Equals)
                }
                _ => Token::new(TokenType::Assign),
            },
            Some(b';') => Token::new(TokenType::SemiColon),
            Some(b'(') => Token::new(TokenType::LParen),
            Some(b')') => Token::new(TokenType::RParen),
            Some(b',') => Token::new(TokenType::Comma),
            Some(b'+') => Token::new(TokenType::Plus),
            Some(b'{') => Token::new(TokenType::LBrace),
            Some(b'}') => Token::new(TokenType::RBrace),
            Some(b'!') => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Token::new(TokenType::NotEquals)
                }
                _ => Token::new(TokenType::Bang),
            },
            Some(b'-') => Token::new(TokenType::Minus),
            Some(b'/') => Token::new(TokenType::Slash),
            Some(b'*') => Token::new(TokenType::Asterisk),
            Some(b'<') => Token::new(TokenType::LT),
            Some(b'>') => Token::new(TokenType::GT),
            Some(c) => {
                if is_letter(c) {
                    skip_read = true;
                    let literal = self.read_identifier();
                    let literal = String::from_utf8_lossy(literal).to_string();
                    match &*literal {
                        "fn" => Token::new(TokenType::Function),
                        "let" => Token::new(TokenType::Let),
                        "if" => Token::new(TokenType::If),
                        "return" => Token::new(TokenType::Return),
                        "true" => Token::new(TokenType::True),
                        "false" => Token::new(TokenType::False),
                        "else" => Token::new(TokenType::Else),
                        _ => Token::new(TokenType::Ident(literal.to_string())),
                    }
                } else if is_digit(c) {
                    skip_read = true;
                    let number = self.read_number();
                    let number = String::from_utf8_lossy(number).to_string();
                    Token::new(TokenType::Int(number))
                } else {
                    Token::new(TokenType::Illegal)
                }
            }
        };

        if !skip_read {
            self.read_char();
        }

        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position])
        }
    }

    fn read_number(&mut self) -> &[u8] {
        let position = self.position;
        while self.ch != None && is_digit(self.ch.unwrap()) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn read_identifier(&mut self) -> &[u8] {
        let position = self.position;
        while self.ch != None && is_letter(self.ch.unwrap()) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch != None && is_whitespace(self.ch.unwrap()) {
            self.read_char();
        }
    }
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

fn is_letter(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

fn is_whitespace(ch: u8) -> bool {
    return ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r';
}
