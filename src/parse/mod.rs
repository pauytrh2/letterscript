use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Program<'a> {
    pub function_name: &'a str,
    pub return_value: i64,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected_type: TokenType) -> Result<Token<'a>, String> {
        match self.current() {
            Some(token) if token._type == expected_type => {
                let tok = token.clone();
                self.advance();
                Ok(tok)
            }
            Some(token) => Err(format!(
                "Expected token {:?}, but got {:?}",
                expected_type, token._type
            )),
            None => Err(format!(
                "Expected token {expected_type}, but got end of input"
            )),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program<'a>, String> {
        // Dear <function_name>,
        self.expect(TokenType::Function)?;
        let func_token = self.expect(TokenType::String)?;
        let func_name = func_token.value.ok_or("Expected function name")?;
        self.expect(TokenType::Comma)?;

        // Regards, <int>.
        self.expect(TokenType::Return)?;
        self.expect(TokenType::Comma)?;
        let int_token = self.expect(TokenType::Int)?;
        let int_str = int_token.value.ok_or("Expected int value")?;
        let int_val: i64 = int_str.parse().map_err(|e| format!("Invalid int: {e}"))?;
        self.expect(TokenType::Period)?;

        Ok(Program {
            function_name: func_name,
            return_value: int_val,
        })
    }
}
