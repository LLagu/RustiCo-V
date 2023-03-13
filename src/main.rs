use std::env;
use std::fs;
mod token_enum; //TODO: just for testing, remove with for loop in main
use token_enum::Token;
use regex::Regex;

pub fn lex(code: &str) -> Vec<Token> {
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let number_re = Regex::new(r"^\d+").unwrap();
    let symbol_re = Regex::new(r"^(=|==|!=|<=|>=|\+|-|\*|/|\(|\)|\{|\}|\[|\]|;|,)").unwrap();

    let keyword_re = Regex::new(r"^(if|else|for|while|do|return)").unwrap();

    let mut tokens = Vec::new();
    let mut code = code;

    while !code.is_empty() {
        code = code.trim_start();

        if let Some(captures) = identifier_re.captures(code) {
            let identifier = captures.get(0).unwrap().as_str().to_owned();
            tokens.push(Token::Identifier(identifier));
            code = &code[captures.get(0).unwrap().end()..];
        } else if let Some(captures) = number_re.captures(code) {
            let number = captures.get(0).unwrap().as_str().to_owned();
            tokens.push(Token::Number(number));
            code = &code[captures.get(0).unwrap().end()..];
        } else if let Some(captures) = symbol_re.captures(code) {
            let symbol = captures.get(0).unwrap().as_str().to_owned();
            tokens.push(Token::Symbol(symbol));
            code = &code[captures.get(0).unwrap().end()..];
        } else if let Some(captures) = keyword_re.captures(code) {
            let keyword = captures.get(0).unwrap().as_str().to_owned();
            tokens.push(Token::Keyword(keyword));
            code = &code[captures.get(0).unwrap().end()..];
        } else {
            panic!("Invalid character: {}", code.chars().next().unwrap());
        }
    }

    tokens
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program_filepath = &args[1];

    println!("In file {}", program_filepath);
    let code= fs::read_to_string(program_filepath).expect("Unable to read the file");

    println!("With text:\n{code}");


    let tokens = lex(&code);

    for token in tokens {
        match token {
            token_enum::Token::Identifier(identifier) => println!("Identifier: {}", identifier),
            token_enum::Token::Number(number) => println!("Number: {}", number),
            token_enum::Token::Symbol(symbol) => println!("Symbol: {}", symbol),
            token_enum::Token::Keyword(keyword) => println!("Keyword: {}", keyword),
        }
    }
}
