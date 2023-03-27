#[derive(Debug)]

pub enum AstNode {
    Expression(usize), //The only expression supported at the moment is an integer constant
    Statement(Box<AstNode>), //Statement(Expression) Right now the only statement is "return"
    Function(String, Box<AstNode>), //Function(functionName, Statement)
    Program(Box<AstNode>), //Program(Function) Later on it will be a list of fucntions
}
