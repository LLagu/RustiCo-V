#[derive(Debug)]
pub enum Token {
    Integer(usize),
    Identifier(String),
    OpenParenthesis,
    ClosedParenthesis,
    OpenBracket,
    ClosedBracket,
    Semicolon,
}
