use std::{fs, time::Instant};

use clap::Parser;
use zenith_lexer::{tokenize_code, types::Token};

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(&args.file).unwrap();

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
