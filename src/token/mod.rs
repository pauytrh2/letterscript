pub enum TokenType {
    MainFunction,
    Return,
    Int,
    Comma,
    Period,
}

pub struct Token {
    pub _type: TokenType,
    pub value: Option<&'static str>,
}
