use std::io::{self, Write};
use std::panic;

#[derive(Debug)]
enum TokenType {
    Number,
    LeftParenthes,
    RightParenthes,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

fn tokenize(expression: String) -> Vec<Token> {
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

struct Parser {
    tokens: Vec<Token>,
    parse_place: usize,
}

impl Parser {
    // Expression ::= PriorityExpression ExpressionTail
    fn parse_expression(&mut self) -> i64 {
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

fn eval(expression: String) -> i64{
    let tokens = tokenize(expression);
    // println!("tokens: {:?}", tokens);
    let mut parser = Parser {
        tokens: tokens,
        parse_place: 0,
    };
    return parser.parse_expression();
}


#[test]
fn test_eval() {
    assert_eq!(eval("2 * (3  -4) + 5 - 6 * 7 ".to_string()), -39);
    assert_eq!(eval("9 +(3 - 1) * 3 + 10 / 2".to_string()), 20);
}


fn main() {
    panic::set_hook(Box::new(|err| {println!("error: {:?}", err)}));
    loop {
        print!("input the expression: ");
        io::stdout().flush().expect("Flust Error!");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok().expect("Read Error!");
        let result = panic::catch_unwind(|| {
            eval(buf.trim().to_string())
        });
        match result {
            Ok(v) => println!("result: {:?}", v),
            _ => (),
        }
    }
}