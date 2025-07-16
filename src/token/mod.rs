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
pub struct Token<'a> {
    pub _type: TokenType,
    pub value: Option<&'a str>,
}
