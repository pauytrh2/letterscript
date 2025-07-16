enum Expecting {
    Nothing,
    String,
    Int,
}

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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenType::Function => "Function",
            TokenType::Return => "Return",
            TokenType::Int => "Int",
            TokenType::String => "String",
            TokenType::Comma => "Comma",
            TokenType::Period => "Period",
        };
        write!(f, "{s}")
    }
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut words = input.split_whitespace().peekable();

    let mut expecting = Expecting::Nothing;

    while let Some(word) = words.next() {
        let (core, punctuation) = strip_punctuation(word);

        match expecting {
            Expecting::String => {
                tokens.push(make_token(TokenType::String, Some(core)));
                expecting = Expecting::Nothing;
            }
            Expecting::Int => {
                tokens.push(make_token(TokenType::Int, Some(core)));
                expecting = Expecting::Nothing;
            }
            Expecting::Nothing => {
                match_token(core, &mut tokens, &mut expecting, &mut words);
            }
        }

        if let Some(punct) = punctuation {
            tokens.push(make_token(punct, None));
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

fn make_token<'a>(_type: TokenType, value: Option<&'a str>) -> Token<'a> {
    Token { _type, value }
}

fn match_token<'a, I>(
    core: &'a str,
    tokens: &mut Vec<Token<'a>>,
    expecting: &mut Expecting,
    words: &mut std::iter::Peekable<I>,
) where
    I: Iterator<Item = &'a str>,
{
    match core {
        "Dear" => {
            tokens.push(make_token(TokenType::Function, None));
            *expecting = Expecting::String;
        }
        "Regards" => {
            tokens.push(make_token(TokenType::Return, None));

            if let Some(next) = words.peek() {
                if next.ends_with(',') {
                    words.next();
                    tokens.push(make_token(TokenType::Comma, None));
                    *expecting = Expecting::Int;
                }
            }
        }
        _ if core.chars().all(|c| c.is_ascii_digit()) => {
            tokens.push(make_token(TokenType::Int, Some(core)));
        }
        _ => {
            tokens.push(make_token(TokenType::String, Some(core)));
        }
    }
}
