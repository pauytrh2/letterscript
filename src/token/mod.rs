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
        } else if core == "Dear" {
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
