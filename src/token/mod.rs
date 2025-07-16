#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Function,
    Return,
    Int,
    String,
    Comma,
    Period,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub _type: TokenType,
    pub value: Option<&'a str>,
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut words = input.split_whitespace().peekable();

    enum Expecting {
        Nothing,
        String,
        Int,
    }

    let mut expecting = Expecting::Nothing;

    while let Some(word) = words.next() {
        let (core, punctuation) = strip_punctuation(word);

        match expecting {
            Expecting::String => {
                tokens.push(Token {
                    _type: TokenType::String,
                    value: Some(core),
                });
                expecting = Expecting::Nothing;
            }
            Expecting::Int => {
                tokens.push(Token {
                    _type: TokenType::Int,
                    value: Some(core),
                });
                expecting = Expecting::Nothing;
            }
            Expecting::Nothing => match core {
                "Dear" => {
                    tokens.push(Token {
                        _type: TokenType::Function,
                        value: None,
                    });
                    expecting = Expecting::String;
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
                            expecting = Expecting::Int;
                        }
                    }
                }
                _ if core.chars().all(|c| c.is_ascii_digit()) => {
                    tokens.push(Token {
                        _type: TokenType::Int,
                        value: Some(core),
                    });
                }
                _ => {
                    tokens.push(Token {
                        _type: TokenType::String,
                        value: Some(core),
                    });
                }
            },
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
