use crate::ast_enum::AstNode;
use crate::token_enum::Token;
use std::iter::Peekable;

pub fn parse_expression(
    tokens: &mut Peekable<std::vec::IntoIter<Token>>,
) -> Result<AstNode, String> {
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::Integer(s) if s == 0) {
        return Err(format!("Expected int but found [add token type here]"));
    } else {
        return Ok(AstNode::Expression(0));
    }
}

pub fn parse_statement(
    tokens: &mut Peekable<std::vec::IntoIter<Token>>,
) -> Result<AstNode, String> {
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;

    // Check return statement
    if !matches!(tok, Token::Identifier(s) if s == "return".to_string()) {
        return Err(format!("Expected return but found [add token type here]"));
    }

    // Get the expression
    let exp = parse_expression(tokens);

    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::Semicolon) {
        return Err(format!("Expected int but found [add token type here]"));
    }

    // TODO: handle errors and remove .unwrap()
    let statement = AstNode::Statement(Box::new(exp.unwrap()));
    Ok(statement)
}

pub fn parse_function(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<AstNode, String> {
    // Check return type int
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::Identifier(s) if s == "int".to_string()) {
        return Err(format!("Expected return but found [add token type here]"));
    }

    // Check main
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::Identifier(s) if s == "main".to_string()) {
        return Err(format!("Expected return but found [add token type here]"));
    }
    // Work in progress
    // let mut function_name = "";
    // match tok {
    //     Token::Identifier(s) => function_name = &s,
    //     _ => {
    //         return Err(format!("Expected return but found [add token type here]"));
    //     }
    // }

    // Parenthesis
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::OpenParenthesis) {
        return Err(format!("Expected return but found [add token type here]"));
    }
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::ClosedParenthesis) {
        return Err(format!("Expected return but found [add token type here]"));
    }

    // Open bracket
    let tok = tokens
        .next()
        .ok_or_else(|| "Expected token but found none".to_string())?;
    if !matches!(tok, Token::OpenBracket) {
        return Err(format!("Expected return but found [add token type here]"));
    }

    // Get the statement
    let statement = parse_statement(tokens);

    let function = AstNode::Function("main".to_owned(), Box::new(statement.unwrap()));
    Ok(function)
}

pub fn parse_program(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<AstNode, String> {
    let function = parse_function(tokens);
    let program = AstNode::Program(Box::new(function.unwrap()));

    if true {
        return Ok(program);
    } else {
        return Err(format!("You should't be here"));
    }
}
