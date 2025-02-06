use crate::token::Token;
use crate::parser::Expr;

const ASM_START: &'static str = "
extern printf
global main

segment .text
main:
";
const ASM_END: &'static str = r#"
    lea rdi, format_int
    mov esi, [rsp]
    call printf
    pop rsi
    ret

segment .data
    format_int: db "%i", 0xD, 0xA, 0
"#;


fn generate_instructions(expr: &Expr, output: &mut String) {
    match expr {
        Expr::Num(n) => {
            output.push_str(&format!("    push {}\n", n));
        }
        Expr::Op { left, op, right } => {
            generate_instructions(left, output);
            generate_instructions(right, output);

            output.push_str("    pop rbx\n");
            output.push_str("    pop rax\n");

            match op {
                Token::Add => output.push_str("    add rax, rbx\n"),
                Token::Sub => output.push_str("    sub rax, rbx\n"),
                Token::Mul => output.push_str("    imul rax, rbx\n"),
                Token::Div => {
                    output.push_str("    cqo\n");  // Weird division thing idk
                    output.push_str("    idiv rbx\n");  
                }
                _ => unreachable!(),
            }

            output.push_str("    push rax\n");
        }
    }
}


pub fn generate_asm(ast: Expr) -> String {
    let mut asm_string = String::new();

    asm_string.push_str(ASM_START);
    generate_instructions(&ast, &mut asm_string);
    asm_string.push_str(ASM_END);

    asm_string
}