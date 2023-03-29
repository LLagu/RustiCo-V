# RustiCo-V
Simple C compiler written in Rust. The assembly generated is RISC-V assembly.

This is a personal project to learn compiler basics. Full of TODOs and temporary solutions. 
Lexer, parser and assembler generator are written from scratch. Works on Linux only.
Currently the only supported type of programs are of type:
```
int main() { 
  return 0; 
}
```

as the implemented grammar is:
```
<program> ::= <function>
<function> ::= "int" <id> "(" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp> ::= <int>
```
The generated assembly is
```
.section .text

.global main
.type main, @function

main:
	li		a0, 0
	jr		ra
```

Prerequisites:

RISC-V toolchain and its dependencies:
https://github.com/riscv-collab/riscv-gnu-toolchain


Loosely following Nora Sandler's C compiler guide written in COBOL (https://norasandler.com/2017/11/29/Write-a-Compiler.html)

