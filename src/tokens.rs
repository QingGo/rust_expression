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
