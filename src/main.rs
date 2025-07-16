use std::{collections::HashMap, env, fs};

mod token;
use token::*;

fn main() {
    let file_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No file path argument provided.\nUsage: letterscript <file.lts>");
        std::process::exit(1);
    });

    let contents = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Could not read the file at path: {file_path}"));

    dbg!(tokenize(&contents));
}

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let keywords = get_keywords();
    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        let (core, punct) = strip_punctuation(word);

        if !core.is_empty() {
            add_token(&mut tokens, core, &keywords);
        }

        if let Some(p) = punct {
            tokens.push(Token {
                _type: p,
                value: None,
            });
        }
    }

    tokens
}

fn strip_punctuation(word: &str) -> (&str, Option<TokenType>) {
    if let Some(stripped) = word.strip_suffix('.') {
        return (stripped, Some(TokenType::Period));
    }
    if let Some(stripped) = word.strip_suffix(',') {
        return (stripped, Some(TokenType::Comma));
    }
    (word, None)
}

fn add_token<'a>(
    tokens: &mut Vec<Token<'a>>,
    word: &'a str,
    keywords: &HashMap<&'static str, TokenType>,
) {
    match keywords.get(word) {
        Some(token_type) => {
            let value = match token_type {
                TokenType::String => Some(word),
                _ => None,
            };

            tokens.push(Token {
                _type: token_type.clone(),
                value,
            });
        }
        None => {
            eprintln!("Unknown token: {word}");
        }
    }
}

fn get_keywords() -> HashMap<&'static str, TokenType> {
    use TokenType::*;

    HashMap::from([
        ("Dear", Function),
        ("main", String),
        ("Regards", Return),
        ("0", Int),
    ])
}
