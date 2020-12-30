pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type: token_type,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    // Delimiters
    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}
