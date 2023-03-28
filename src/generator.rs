use crate::ast_enum::AstNode;
use std::{fs, process::Command};

fn print_to_file(file: &str, file_name: &str) {
    fs::write(format!("{}.s", file_name), file);
}

fn generate_assembly(ast: AstNode) -> String {
    match ast {
        AstNode::Program(f) => {
            let code = generate_assembly(*f);
            format!("{}{}", ".section .text\n\n", code)
        }
        AstNode::Function(function_name, e) => {
            let code = generate_assembly(*e);
            let function_string = format!(
                ".global {}\n.type {}, @function\n\n{}:\n",
                function_name, function_name, function_name
            );
            format!("{}{}", function_string, code)
        }
        AstNode::Statement(s) => {
            let code = generate_assembly(*s);
            format!("\tli\t\ta0, {}\n\tjr\t\tra", code)
        }
        AstNode::Expression(i) => return i.to_string(), // _ => println!("Program not found"),
    }
}

pub fn generate_executable(code_string: AstNode, file_name: String) {
    let assembly = generate_assembly(code_string);
    print_to_file(&assembly, &file_name);

    let output = Command::new("")
        .arg(format!("/src/out/{}.s", &file_name))
        .arg("-o")
        .arg(format!("{}", &file_name))
        .output()
        .expect("failed to convert the assembly to an executable");

        println!("status: {}", output.status);
        println!("stdout: {:?}", String::from_utf8(output.stdout));
        println!("stderr: {:?}", String::from_utf8(output.stderr));   
}
