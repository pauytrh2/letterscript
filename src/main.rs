use std::{collections::HashMap, env, fs};

mod token;
use token::*;

fn main() {
    let file_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No file path argument provided.\nUsage: letterscript <file.lts>");
        std::process::exit(1);
    });

    let contents = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("Could not read file at '{file_path}': {err}");
        std::process::exit(1);
    });

    dbg!(tokenize(&contents));
}

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let keywords = get_keywords();
    let mut tokens = Vec::new();

    let mut words = input.split_whitespace().peekable();
    let mut expect_string = false;
    let mut expect_int = false;

    while let Some(word) = words.next() {
        let (core, punctuation) = strip_punctuation(word);

        if expect_string {
            tokens.push(Token {
                _type: TokenType::String,
                value: Some(core),
            });
            expect_string = false;
        } else if expect_int {
            tokens.push(Token {
                _type: TokenType::Int,
                value: Some(core),
            });
            expect_int = false;
        } else {
            match core {
                "Dear" => {
                    tokens.push(Token {
                        _type: TokenType::Function,
                        value: None,
                    });
                    expect_string = true;
                }
                "Regards" => {
                    tokens.push(Token {
                        _type: TokenType::Return,
                        value: None,
                    });

                    if let Some(next) = words.peek() {
                        if next.ends_with(',') {
                            words.next();
                            tokens.push(Token {
                                _type: TokenType::Comma,
                                value: None,
                            });
                            expect_int = true;
                        }
                    }
                }
                _ => {
                    if let Some(token_type) = keywords.get(core) {
                        tokens.push(Token {
                            _type: token_type.clone(),
                            value: Some(core),
                        });
                    } else {
                        eprintln!("Unknown token: {core}");
                    }
                }
            }
        }

        if let Some(p) = punctuation {
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

fn get_keywords() -> HashMap<&'static str, TokenType> {
    use TokenType::*;

    HashMap::from([
        ("Dear", Function),
        ("main", String),
        ("Regards", Return),
        ("0", Int),
    ])
}
