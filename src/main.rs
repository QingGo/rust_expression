mod parser;
mod tokenize;

use parser::new_parser;
use std::io::{self, Write};
use std::panic;
use tokenize::tokenize;

fn eval(expression: String) -> i64 {
    let tokens = tokenize(expression);
    // println!("tokens: {:?}", tokens);
    let mut parser = new_parser(tokens);
    return parser.parse_expression();
}

#[test]
fn test_eval() {
    assert_eq!(eval("2 * (3  -4) + 5 - 6 * 7 ".to_string()), -39);
    assert_eq!(eval("9 +(3 - 1) * 3 + 10 / 2".to_string()), 20);
}

fn main() {
    panic::set_hook(Box::new(|err| println!("error: {:?}", err)));
    loop {
        print!("input the expression: ");
        io::stdout().flush().expect("Flust Error!");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok().expect("Read Error!");
        let result = panic::catch_unwind(|| eval(buf.trim().to_string()));
        match result {
            Ok(v) => println!("result: {:?}", v),
            _ => (),
        }
    }
}
