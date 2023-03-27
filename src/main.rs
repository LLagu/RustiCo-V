use std::env;
use std::fs;
mod ast_enum;
mod generator;
mod lexer;
mod parser;
mod token_enum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_filepath = &args[1];

    println!("In file {}", program_filepath);
    let code = fs::read_to_string(program_filepath).expect("Unable to read the file");

    println!("With text:\n{code}");

    let mut tokens = lexer::lex(&code);
    println!("{:#?}", tokens);

    let ast = parser::parse_program(&mut tokens);
    println!("{:#?}", ast);

    generator::generate_assembly(ast.unwrap(), "return_constant".to_string());
}
