use std::{env, fs, process::Command};

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

    let asm_code = to_asm(tokenize(&contents));

    fs::write("output.asm", asm_code).expect("Unable to write to file");

    Command::new("nasm")
        .args(["-f", "elf64"])
        .args(["-o", "output.o"])
        .arg("output.asm")
        .spawn()
        .expect("nasm failed")
        .wait()
        .expect("nasm wait failed");
    Command::new("ld")
        .args(["-o", "output"])
        .arg("output.o")
        .spawn()
        .expect("ld failed")
        .wait()
        .expect("ld wait failed");
}

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
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
            if core == "Dear" {
                tokens.push(Token {
                    _type: TokenType::Function,
                    value: None,
                });
                expect_string = true;
            } else if core == "Regards" {
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
            } else if core.chars().all(|c| c.is_ascii_digit()) {
                tokens.push(Token {
                    _type: TokenType::Int,
                    value: Some(core),
                });
            } else {
                tokens.push(Token {
                    _type: TokenType::String,
                    value: Some(core),
                });
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

fn to_asm(tokens: Vec<Token>) -> String {
    let mut output = String::from("global _start\n\n_start:\n");
    let exit_code_line = format!(
        "    mov edi, {}\n",
        find_exit_code(tokens.as_slice()).expect("Failed to find exit code")
    );

    for token in tokens {
        if token._type == TokenType::Return {
            output.push_str("    mov eax, 60\n");
            output.push_str(&exit_code_line);
            output.push_str("    syscall");
        }
    }

    output
}

fn find_exit_code(tokens: &[Token]) -> Option<i32> {
    for i in 0..tokens.len() {
        if tokens[i]._type == TokenType::Return {
            if let Some(Token {
                _type: TokenType::Comma,
                ..
            }) = tokens.get(i + 1)
            {
                if let Some(Token {
                    _type: TokenType::Int,
                    value: Some(v),
                }) = tokens.get(i + 2)
                {
                    return v.parse().ok();
                }
            }
        }
    }

    None
}
