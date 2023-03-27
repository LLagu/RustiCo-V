use crate::ast_enum::AstNode;
use std::{fs, path::PathBuf};

fn print_to_file(file: &str, file_name: String){
    fs::write(format!("{}.s", file_name), file);
}

fn explore_ast(ast: AstNode) -> String {
    match ast {
        AstNode::Program(f) => {
            let code = explore_ast(*f);
            format!("{}{}", ".section .text\n\n", code)
        }
        AstNode::Function(function_name, e) => {
            let code = explore_ast(*e);
            let function_string = format!(
                ".global {}\n.type {}, @function\n\n{}:\n",
                function_name, function_name, function_name
            );
            format!("{}{}", function_string, code)
        }
        AstNode::Statement(s) => {
            let code = explore_ast(*s);
            format!("\tli\t\ta0, {}\n\tjr\t\tra", code)
        }
        AstNode::Expression(i) => return i.to_string(), // _ => println!("Program not found"),
    }
}

pub fn generate_assembly(code_string: AstNode, file_name: String) {
    print_to_file(&explore_ast(code_string), file_name);
}
