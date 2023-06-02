use std::{fs, time::Instant};

use zenith_lexer::{tokenize_code, types::Token};

fn main() {
    let code = fs::read_to_string("code.zen").unwrap();

    let start = Instant::now();
    let tokens = tokenize_code(&code);
    let lex_time = start.elapsed();

    for tok in tokens {
        // let (start, end) = tok.get_position();
        match tok.strip_token() {
            Token::Whitespace(_) => {}
            _ => println!("{}", tok),
        }
    }

    println!("Lex time: {:?}", lex_time);
}
