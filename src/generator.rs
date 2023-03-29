use crate::ast_enum::AstNode;
use std::io::Error;
use std::{fs, process::Command};

use std::env;

fn print_to_out_folder(file: &str, file_name: &str) -> Result<(), Error> {
    let cwd = env::current_dir()?;
    let path = cwd.join("out").join(format!("{}.s", file_name));
    fs::create_dir_all(path.parent().unwrap())?; // create the directory if it doesn't exist
    fs::write(&path, file)?;
    Ok(())
}

pub fn generate_assembly(ast: AstNode) -> String {
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
    print_to_out_folder(&assembly, &file_name);

    let output = Command::new("riscv64-linux-gnu-gcc")
        .arg("-c")
        .arg(format!("out/{}.s", &file_name))
        .arg("-o")
        .arg(format!("out/{}.o", &file_name))
        .output()
        .expect("failed to convert the assembly to an executable");

        println!("status: {}", output.status);
        println!("stdout: {:?}", String::from_utf8(output.stdout));
        println!("stderr: {:?}", String::from_utf8(output.stderr));  
}
