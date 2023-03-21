use std::env;
use std::fs;
mod lexer;
mod token_enum; //TODO: just for testing, remove with for loop in main
use token_enum::Token;

fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_digit(10) {
            let mut n = c.to_digit(10).unwrap() as usize;
            chars.next();
            while let Some(&d) = chars.peek() {
                if d.is_digit(10) {
                    n = n * 10 + d.to_digit(10).unwrap() as usize;
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Integer(n));
        } else if c.is_alphabetic() {
            let mut identifier = String::new();
            identifier.push(c);
            chars.next();
            while let Some(&d) = chars.peek() {
                if d.is_alphanumeric() || d == '_' {
                    identifier.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Identifier(identifier));
        } else {
            match c {
                ')' => { tokens.push(Token::ClosedParenthesis); chars.next(); },
                '}' => { tokens.push(Token::ClosedBracket); chars.next(); },
                '(' => { tokens.push(Token::OpenParenthesis); chars.next(); },
                '{' => { tokens.push(Token::OpenBracket); chars.next(); },
                ';' => { tokens.push(Token::Semicolon); chars.next(); },
                _ => panic!("Unexpected character: {}", c),
            }
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
    println!("{:#?}", tokens);
    // for token in tokens {
    //     match token {
    //         token_enum::Token::Identifier(identifier) => println!("Identifier: {}", identifier),
    //         token_enum::Token::Number(number) => println!("Number: {}", number),
    //         token_enum::Token::Symbol(symbol) => println!("Symbol: {}", symbol),
    //         token_enum::Token::Keyword(keyword) => println!("Keyword: {}", keyword),
    //     }
    // }
}
