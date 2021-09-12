#[derive(Debug)]
pub enum TokenType {
    Number,
    LeftParenthes,
    RightParenthes,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn tokenize(expression: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut temp = String::new();
    for single_char in expression.chars() {
        if temp.len() != 0 && "+-*/() ".contains(single_char) {
            tokens.push(Token {
                token_type: TokenType::Number,
                value: temp,
            });
            temp = String::new();
        }
        match single_char {
            ' ' => continue,
            '+' => tokens.push(Token {
                token_type: TokenType::Plus,
                value: "".to_string(),
            }),
            '-' => tokens.push(Token {
                token_type: TokenType::Minus,
                value: "".to_string(),
            }),
            '*' => tokens.push(Token {
                token_type: TokenType::Multiply,
                value: "".to_string(),
            }),
            '/' => tokens.push(Token {
                token_type: TokenType::Divide,
                value: "".to_string(),
            }),
            '(' => tokens.push(Token {
                token_type: TokenType::LeftParenthes,
                value: "".to_string(),
            }),
            ')' => tokens.push(Token {
                token_type: TokenType::RightParenthes,
                value: "".to_string(),
            }),
            _ => temp.push(single_char),
        }
    }
    if temp.len() != 0 {
        tokens.push(Token {
            token_type: TokenType::Number,
            value: temp,
        });
    }
    return tokens;
}
