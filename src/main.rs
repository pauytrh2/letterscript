use std::{env, fs};
use token::*;

mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!(
                "Error: No file path argument provided.\nPlease provide a file path.\nletterscript <file.lts>"
            );
            std::process::exit(1);
        }
    };

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    dbg!(contents);
}

fn tokenize(input: &str) -> Vec<Token> {
    // temp
    vec![
        Token {
            _type: TokenType::MainFunction,
            value: None,
        },
        Token {
            _type: TokenType::Return,
            value: None,
        },
        Token {
            _type: TokenType::Int,
            value: Some("0"),
        },
    ]
}
