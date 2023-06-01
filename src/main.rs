use std::{fs, time::Instant};

mod lexer;
use lexer::{lex_code, Token};

fn main() {
    let code = fs::read_to_string("code.jiz").unwrap();

    let start = Instant::now();
    let tokens = lex_code(&code);
    let lex_time = start.elapsed();

    for tok in tokens {
        match tok.get_token() {
            Token::Whitespace(_) => {}
            _ => println!("{}", tok),
        }
    }

    println!("Lex time: {:?}", lex_time);
}
