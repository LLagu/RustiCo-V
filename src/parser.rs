use crate::token_enum::Token;
use crate::ast_enum::AstNode;
use std::iter::Peekable;

pub fn parse_statement(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<AstNode, String> {
    let tok = tokens.next().ok_or_else(|| "Expected token but found none".to_string())?;

    if !matches!(tok, Token::Identifier(s) if s == "return".to_string()) {
        return Err(format!("Expected return but found [add token type here]"));
    }
    let tok = tokens.next().ok_or_else(|| "Expected token but found none".to_string())?;
    // if tok.type_ != "INT" {
    //     return Err(format!("Expected INT but found {}", tok.type_));
    // }
    if !matches!(tok, Token::Integer(s) if s == 0) {
        return Err(format!("Expected int but found [add token type here]"));
    }
    // let exp = parse_exp(tokens)?;
    // let statement = Statement::Return(exp);

    let tok = tokens.next().ok_or_else(|| "Expected token but found none".to_string())?;
    // if tok.type_ != "SEMICOLON" {
    //     return Err(format!("Expected SEMICOLON but found {}", tok.type_));
    // }
    if !matches!(tok, Token::Semicolon) {
        return Err(format!("Expected int but found [add token type here]"));
    }

    let statement= AstNode::Statement(Box::new(AstNode::Constant(2)));
    Ok(statement)
}
