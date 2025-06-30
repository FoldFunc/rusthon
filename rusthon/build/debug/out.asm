global _start
section .text
_start:
    sub rsp, 64
    mov rax, 10
    ; store var x on stack 
    mov [rsp - 8], rax
    mov rax, 11
    ; store var y on stack 
    mov [rsp - 16], rax
    mov rax, 90
    mov rdi, rax
    mov rax, 60
    syscall
