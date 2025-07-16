use crate::token::{Token, TokenType};

pub fn to_asm(tokens: Vec<Token>) -> String {
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
