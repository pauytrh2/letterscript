#[derive(Debug, Clone)]
pub enum TokenType {
    MainFunction,
    Return,
    Int,
    Comma,
    Period,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub _type: TokenType,
    pub value: Option<&'a str>,
}
