# RustiCo-V
Simple C compiler written in Rust. The assembly generated will be RISC-V assembly.

This is a personal project to learn compiler basics. Both lexer and parser are written from scratch.
Currently the only supported type of programs are of type:

"int main() { return 0; }"

as the implemented grammar is:
"<program> ::= <function>
<function> ::= "int" <id> "(" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp> ::= <int>"

Assembly generation coming soon.
Full of TODOs and temporary solutions.

Following Nora Sandler's C compiler guide (written in COBOL)
https://norasandler.com/2017/11/29/Write-a-Compiler.html
