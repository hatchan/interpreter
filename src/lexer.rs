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
            Some(b'=') => Token::new(TokenType::Assign),
            Some(b';') => Token::new(TokenType::SemiColon),
            Some(b'(') => Token::new(TokenType::LParen),
            Some(b')') => Token::new(TokenType::RParen),
            Some(b',') => Token::new(TokenType::Comma),
            Some(b'+') => Token::new(TokenType::Plus),
            Some(b'{') => Token::new(TokenType::LBrace),
            Some(b'}') => Token::new(TokenType::RBrace),
            Some(c) => {
                if is_letter(c) {
                    skip_read = true;
                    let literal = self.read_identifier();
                    let literal = String::from_utf8(literal).unwrap();
                    match &*literal {
                        "fn" => Token::new(TokenType::Function),
                        "let" => Token::new(TokenType::Let),
                        _ => Token::new(TokenType::Ident(literal.to_string())),
                    }
                } else if is_digit(c) {
                    skip_read = true;
                    let number = self.read_number();
                    let number = String::from_utf8(number).unwrap();
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

    fn read_number(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        while self.ch != None {
            let c = self.ch.unwrap();
            if is_digit(c) {
                buf.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        buf
    }

    fn read_identifier(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];

        while self.ch != None {
            let c = self.ch.unwrap();
            if is_letter(c) {
                buf.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        buf
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
