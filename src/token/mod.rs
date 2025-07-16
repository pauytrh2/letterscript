#[derive(Debug)]
pub enum TokenType {
    MainFunction,
    Return,
    Int,
    Comma,
    Period,
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<&'static str>,
}
