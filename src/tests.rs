#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::TokenType;

    #[test]
    fn test1() {
        let input = String::from("=+(){},;");
        let tests = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::Comma,
            TokenType::SemiColon,
            TokenType::EOF,
        ];

        let mut l = lexer::Lexer::new(&input);

        for expected_type in tests.iter() {
            let token = l.next_token();
            assert_eq!(&token.token_type, expected_type);
        }
    }

    #[test]
    fn test2() {
        let input = String::from(
            "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
            return true;
            } else {
            return false;
            }",
        );
        let tests = vec![
            TokenType::Let,
            TokenType::Ident(String::from("five")),
            TokenType::Assign,
            TokenType::Int(String::from("5")),
            TokenType::SemiColon,
            TokenType::Let,
            TokenType::Ident(String::from("ten")),
            TokenType::Assign,
            TokenType::Int(String::from("10")),
            TokenType::SemiColon,
            TokenType::Let,
            TokenType::Ident(String::from("add")),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident(String::from("x")),
            TokenType::Comma,
            TokenType::Ident(String::from("y")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident(String::from("x")),
            TokenType::Plus,
            TokenType::Ident(String::from("y")),
            TokenType::SemiColon,
            TokenType::RBrace,
            TokenType::SemiColon,
            TokenType::Let,
            TokenType::Ident(String::from("result")),
            TokenType::Assign,
            TokenType::Ident(String::from("add")),
            TokenType::LParen,
            TokenType::Ident(String::from("five")),
            TokenType::Comma,
            TokenType::Ident(String::from("ten")),
            TokenType::RParen,
            TokenType::SemiColon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int(String::from("5")),
            TokenType::SemiColon,
            TokenType::Int(String::from("5")),
            TokenType::LT,
            TokenType::Int(String::from("10")),
            TokenType::GT,
            TokenType::Int(String::from("5")),
            TokenType::SemiColon,
            TokenType::If,
            TokenType::LParen,
            TokenType::Int(String::from("5")),
            TokenType::LT,
            TokenType::Int(String::from("10")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::True,
            TokenType::SemiColon,
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::False,
            TokenType::SemiColon,
            TokenType::RBrace,
            TokenType::EOF,
        ];

        let mut l = lexer::Lexer::new(&input);

        for expected_type in tests.iter() {
            let token = l.next_token();
            assert_eq!(&token.token_type, expected_type);
        }
    }
}
