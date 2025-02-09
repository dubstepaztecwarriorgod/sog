extern printf
global main

segment .text
main:
    push 100
    push 2
    push 55
    pop rbx
    pop rax
    imul rax, rbx
    push rax
    pop rbx
    pop rax
    sub rax, rbx
    push rax

    lea rdi, format_int
    mov esi, [rsp]
    call printf
    pop rsi
    ret

segment .data
    format_int: db "%i", 0xD, 0xA, 0
