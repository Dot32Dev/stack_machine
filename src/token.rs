pub enum Token {
    Integer(i32),
    Variable(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    LeftParen,
    RightParen,
    Return,
    Semicolon,
    Var,
    Equals,
    EOF
}
