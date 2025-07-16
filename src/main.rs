use std::collections::HashMap;
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
    dbg!(tokenize(contents.as_str()));
}

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    // At the end it should return something like:
    //
    // vec![
    //     Token {
    //         _type: TokenType::MainFunction,
    //         value: None,
    //     },
    //     Token {
    //         _type: TokenType::Return,
    //         value: None,
    //     },
    //     Token {
    //         _type: TokenType::Int,
    //         value: Some("0"),
    //     },
    // ]

    let keywords = get_keywords();

    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        dbg!(word);

        if let Some(token_type) = keywords.get(word) {
            tokens.push(Token {
                _type: token_type.clone(),
                value: Some(word),
            });
        } else {
            // Unknown tokens, will implement later
        }
    }

    tokens
}

fn get_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("Dear main", TokenType::MainFunction);
    keywords.insert("return", TokenType::Return);
    keywords.insert("0", TokenType::Int);

    keywords
}
