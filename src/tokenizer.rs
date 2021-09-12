use crate::parser::ITokenizer;
use crate::tokens::{Token, TokenType};
use std::collections::HashMap;

#[derive(Debug)]
enum TokenizerState {
    InsideNumber,
    Operator,
    Parenthes,
    Space,
}

#[derive(Debug)]
pub struct Tokenizer {
    expression: String,
    char_index: usize,
    tokens_map: HashMap<char, Token>,
    last_token: Option<Token>,
    tokenizer_state: TokenizerState,
}

pub fn new_tokenize(expression: String) -> Box<dyn ITokenizer> {
    let mut tokens_map = HashMap::new();
    tokens_map.insert(
        '+',
        Token {
            token_type: TokenType::Plus,
            value: "".to_string(),
        },
    );
    tokens_map.insert(
        '-',
        Token {
            token_type: TokenType::Minus,
            value: "".to_string(),
        },
    );
    tokens_map.insert(
        '*',
        Token {
            token_type: TokenType::Multiply,
            value: "".to_string(),
        },
    );
    tokens_map.insert(
        '/',
        Token {
            token_type: TokenType::Divide,
            value: "".to_string(),
        },
    );
    tokens_map.insert(
        '(',
        Token {
            token_type: TokenType::LeftParenthes,
            value: "".to_string(),
        },
    );
    tokens_map.insert(
        ')',
        Token {
            token_type: TokenType::RightParenthes,
            value: "".to_string(),
        },
    );

    let tokenizer = Tokenizer {
        // 对末尾的空格还不能很好地处理
        expression: expression.trim().to_string(),
        char_index: 0,
        tokens_map: tokens_map,
        last_token: None,
        tokenizer_state: TokenizerState::Space,
    };
    // println!("tokens: {:?}", tokenizer);
    Box::new(tokenizer)
}

impl Tokenizer {
    fn _seek(&mut self) -> Token {
        let mut temp_value = String::new();
        let mut token = Token {
            token_type: TokenType::Minus,
            value: "".to_string(),
        };
        while self.char_index < self.expression.len() {
            let asc_letter = self.expression.as_bytes()[self.char_index] as char;
            match self.tokenizer_state {
                TokenizerState::InsideNumber => {
                    if self.tokens_map.contains_key(&asc_letter)
                        || asc_letter == ' '
                        || self.char_index == self.expression.len() - 1
                    {
                        if self.char_index == self.expression.len() - 1 {
                            temp_value.push(asc_letter);
                            self.char_index += 1;
                        }
                        token = Token {
                            token_type: TokenType::Number,
                            value: temp_value,
                        };
                        self.tokenizer_state = TokenizerState::Space;
                        break;
                    } else {
                        self.char_index += 1;
                        self.tokenizer_state = TokenizerState::InsideNumber;
                        temp_value.push(asc_letter);
                    }
                }
                TokenizerState::Space | TokenizerState::Parenthes => {
                    match self.tokens_map.get(&asc_letter) {
                        Some(_token) => {
                            if "()".contains(asc_letter) {
                                self.tokenizer_state = TokenizerState::Parenthes;
                            } else {
                                self.tokenizer_state = TokenizerState::Operator;
                            }
                            token = _token.clone();
                            self.char_index += 1;
                            break;
                        }
                        None => {
                            self.char_index += 1;
                            if asc_letter == ' ' {
                                self.tokenizer_state = TokenizerState::Space;
                                continue;
                            }
                            // 最后一位是数字的情况
                            if self.char_index == self.expression.len() {
                                temp_value.push(asc_letter);
                                token = Token {
                                    token_type: TokenType::Number,
                                    value: temp_value,
                                };
                                break;
                            }
                            self.tokenizer_state = TokenizerState::InsideNumber;
                            temp_value.push(asc_letter);
                        }
                    }
                }
                TokenizerState::Operator => {
                    self.char_index += 1;
                    if "()".contains(asc_letter) {
                        self.tokenizer_state = TokenizerState::Parenthes;
                        token = self.tokens_map.get(&asc_letter).unwrap().clone();
                        break;
                    }
                    if asc_letter == ' ' {
                        self.tokenizer_state = TokenizerState::Space;
                        continue;
                    }
                    // 最后一位是数字的情况
                    if self.char_index == self.expression.len() {
                        temp_value.push(asc_letter);
                        token = Token {
                            token_type: TokenType::Number,
                            value: temp_value,
                        };
                        break;
                    }
                    self.tokenizer_state = TokenizerState::InsideNumber;
                    temp_value.push(asc_letter);
                }
            }
        }
        println!("token {:?} {}", token, self.char_index);
        token
    }
}

impl ITokenizer for Tokenizer {
    fn has_token(&self) -> bool {
        self.char_index < self.expression.len()
    }

    fn pop(&mut self) -> Token {
        // ownership problem, if use the reference of token then can't change it laterly
        let token = Box::new(self.seek().clone());
        self.last_token = None;
        *token
    }

    fn seek(&mut self) -> Token {
        match &self.last_token {
            Some(_token) => _token.clone(),
            None => {
                let token = self._seek().clone();
                self.last_token = Some(token.clone());
                return token;
            }
        }
    }
}
