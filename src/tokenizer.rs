use crate::parser::ITokenizer;
use crate::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Tokenizer {
    // expression: String,
    tokens: Vec<Token>,
    token_index: usize,
}

pub fn new_tokenize(expression: String) -> Box<dyn ITokenizer> {
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
    let tokenizer = Tokenizer {
        // expression: expression,
        tokens: tokens,
        token_index: 0,
    };
    println!("tokens: {:?}", tokenizer);
    Box::new(tokenizer)
}

impl ITokenizer for Tokenizer {
    fn has_token(&self) -> bool {
        self.token_index < self.tokens.len()
    }

    fn pop(&mut self) -> &Token {
        let token = &self.tokens[self.token_index];
        self.token_index += 1;
        token
    }

    fn seek(&self) -> &Token {
        &self.tokens[self.token_index]
    }
}
