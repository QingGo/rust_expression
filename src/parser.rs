use crate::tokens::{Token, TokenType};
use std::panic;

pub trait ITokenizer {
    fn has_token(&self) -> bool;
    fn pop(&mut self) -> Token;
    fn seek(&mut self) -> Token;
}

pub struct Parser {
    tokenizer: Box<dyn ITokenizer>,
}

pub fn new_parser(tokenizer: Box<dyn ITokenizer>) -> Parser {
    Parser {
        tokenizer: tokenizer,
    }
}

impl Parser {
    // Expression ::= PriorityExpression ExpressionTail
    pub fn parse_expression(&mut self) -> i64 {
        if !self.tokenizer.has_token() {
            panic!("tokens all used when parse_expression")
            // return 0;
        }
        let input = self.parse_priority_expression();
        return self.parse_expression_tail(input);
    }

    // ExpressionTail ::= ([+-] PriorityExpression ExpressionTail)?
    fn parse_expression_tail(&mut self, input: i64) -> i64 {
        if !self.tokenizer.has_token() {
            return input;
        }
        match self.tokenizer.seek().token_type {
            TokenType::Plus => {
                self.tokenizer.pop();
                let opt_num = self.parse_priority_expression();
                return self.parse_expression_tail(input + opt_num);
            }
            TokenType::Minus => {
                self.tokenizer.pop();
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
        if !self.tokenizer.has_token() {
            panic!("tokens all used when parse_term")
        }
        let input = self.parse_factor();
        return self.parse_priority_expression_tail(input);
    }

    // PriorityExpressionTail ::= ([*/] Factor PriorityExpressionTail)?
    fn parse_priority_expression_tail(&mut self, input: i64) -> i64 {
        if !self.tokenizer.has_token() {
            return input;
        }
        match self.tokenizer.seek().token_type {
            TokenType::Multiply => {
                self.tokenizer.pop();
                let opt_num = self.parse_factor();
                return self.parse_priority_expression_tail(input * opt_num);
            }
            TokenType::Divide => {
                self.tokenizer.pop();
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
        if !self.tokenizer.has_token() {
            panic!("tokens all used when parse_value")
        }
        match self.tokenizer.seek().token_type {
            TokenType::Number => {
                let value: i64 = self.tokenizer.seek().value.parse().unwrap();
                self.tokenizer.pop();
                return value;
            }
            TokenType::LeftParenthes => {
                self.tokenizer.pop();
                let value = self.parse_expression();
                self.tokenizer.pop();
                return value;
            }
            _ => {
                panic!(
                    "unexpect token when parse_value {:?}",
                    self.tokenizer.seek()
                )
            }
        }
    }
}
