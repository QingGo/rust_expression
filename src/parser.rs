use crate::tokenize::{Token, TokenType};
use std::panic;

pub struct Parser {
    tokens: Vec<Token>,
    parse_place: usize,
}

pub fn new_parser(tokens: Vec<Token>) -> Parser {
    Parser {
        tokens: tokens,
        parse_place: 0,
    }
}

impl Parser {
    // Expression ::= PriorityExpression ExpressionTail
    pub fn parse_expression(&mut self) -> i64 {
        if self.tokens.len() == self.parse_place {
            panic!("tokens all used when parse_expression")
        }
        let input = self.parse_priority_expression();
        return self.parse_expression_tail(input);
    }

    // ExpressionTail ::= ([+-] PriorityExpression ExpressionTail)?
    fn parse_expression_tail(&mut self, input: i64) -> i64 {
        if self.tokens.len() == self.parse_place {
            return input;
        }
        match self.tokens[self.parse_place].token_type {
            TokenType::Plus => {
                self.parse_place += 1;
                let opt_num = self.parse_priority_expression();
                return self.parse_expression_tail(input + opt_num);
            }
            TokenType::Minus => {
                self.parse_place += 1;
                let opt_num = self.parse_priority_expression();
                return self.parse_expression_tail(input - opt_num);
            }
            _ => {
                return input;
            }
        }
    }

    // PriorityExpression ::= PriorityExpression PriorityExpressionTail
    fn parse_priority_expression(&mut self) -> i64 {
        if self.tokens.len() == self.parse_place {
            panic!("tokens all used when parse_term")
        }
        let input = self.parse_factor();
        return self.parse_priority_expression_tail(input);
    }

    // PriorityExpressionTail ::= ([*/] Factor PriorityExpressionTail)?
    fn parse_priority_expression_tail(&mut self, input: i64) -> i64 {
        if self.tokens.len() == self.parse_place {
            return input;
        }
        match self.tokens[self.parse_place].token_type {
            TokenType::Multiply => {
                self.parse_place += 1;
                let opt_num = self.parse_factor();
                return self.parse_priority_expression_tail(input * opt_num);
            }
            TokenType::Divide => {
                self.parse_place += 1;
                let opt_num = self.parse_factor();
                return self.parse_priority_expression_tail(input / opt_num);
            }
            _ => {
                return input;
            }
        }
    }

    // Factor ::= Number | "(" Expression ")"
    fn parse_factor(&mut self) -> i64 {
        if self.tokens.len() == self.parse_place {
            panic!("tokens all used when parse_value")
        }
        match self.tokens[self.parse_place].token_type {
            TokenType::Number => {
                let value: i64 = self.tokens[self.parse_place].value.parse().unwrap();
                self.parse_place += 1;
                return value;
            }
            TokenType::LeftParenthes => {
                self.parse_place += 1;
                let value = self.parse_expression();
                self.parse_place += 1;
                return value;
            }
            _ => {
                panic!(
                    "unexpect token when parse_value {:?}",
                    self.tokens[self.parse_place]
                )
            }
        }
    }
}
