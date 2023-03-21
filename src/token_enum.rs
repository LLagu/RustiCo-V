#[derive(Debug)]
pub enum Token {
    Integer(usize),
    Identifier(String),
    ClosedParenthesis, 
    ClosedBracket,  
    OpenParenthesis, 
    OpenBracket, 
    Semicolon,
}