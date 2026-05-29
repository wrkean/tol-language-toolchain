#![allow(unused)]

use std::fs;

use clap::Parser;
use lexer::Lexer;
use shared::Args;

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.run();

    for token in tokens {
        println!("{} => {:?}", token.lexeme(), token.kind());
    }
}
