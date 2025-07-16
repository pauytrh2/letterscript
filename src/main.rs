use std::collections::HashMap;
use std::{env, fs};

mod token;
use token::*;

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
    let keywords = get_keywords();
    let mut tokens = Vec::new();

    for raw_word in input.split_whitespace() {
        let mut word = raw_word;

        // Handle trailing punctuation (e.g. "return," becomes "return" and ",")
        while !word.is_empty() {
            if word.ends_with('.') {
                let trimmed = &word[..word.len() - 1];
                if !trimmed.is_empty() {
                    add_token(&mut tokens, trimmed, &keywords);
                }
                tokens.push(Token {
                    _type: TokenType::Period,
                    value: None,
                });
                break;
            } else if word.ends_with(',') {
                let trimmed = &word[..word.len() - 1];
                if !trimmed.is_empty() {
                    add_token(&mut tokens, trimmed, &keywords);
                }
                tokens.push(Token {
                    _type: TokenType::Comma,
                    value: None,
                });
                break;
            } else {
                add_token(&mut tokens, word, &keywords);
                break;
            }
        }
    }

    tokens
}

fn add_token<'a>(
    tokens: &mut Vec<Token<'a>>,
    word: &'a str,
    keywords: &HashMap<&'static str, TokenType>,
) {
    if let Some(token_type) = keywords.get(word) {
        tokens.push(Token {
            _type: token_type.clone(),
            value: Some(word),
        });
    } else {
        eprintln!("Unknown token: {}", word); // TODO make this error handeling better
    }
}

fn get_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("Dear", TokenType::Function);
    keywords.insert("main", TokenType::String);
    keywords.insert("return", TokenType::Return);
    keywords.insert("0", TokenType::Int);

    keywords
}
