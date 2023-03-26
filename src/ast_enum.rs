#[derive(Debug)]

pub enum AstNode
{
    Constant(usize),
    Statement(Box<AstNode>),
    Function(String, Box<AstNode>),
    Program(Box<AstNode>),
}