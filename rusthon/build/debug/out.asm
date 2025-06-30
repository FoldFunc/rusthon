global _start
section .text
_start:
    mov rax, 10
    ; store var x on stack 
    push rax
    mov rax, 90
    mov rdi, rax
    mov rax, 60
    syscall
