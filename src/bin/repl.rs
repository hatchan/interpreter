use std::io;

fn main() {
    eprintln!("Starting repl...");

    loop {
        eprint!("Input monkey code: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => dump_tokens(&input),
            Err(e) => eprintln!("Error: {}", e),
        }
        eprintln!("");
    }
}

fn dump_tokens(input: &String) {
    let mut lexer = monkey::lexer::Lexer::new(&input);
    loop {
        let token = lexer.next_token();
        if token.token_type == monkey::token::TokenType::EOF {
            break;
        }
        eprintln!("Token: {:?}", token.token_type);
    }
}
