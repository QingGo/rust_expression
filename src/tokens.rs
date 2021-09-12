#[derive(Debug, Clone)]
pub enum TokenType {
    Number,
    LeftParenthes,
    RightParenthes,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
