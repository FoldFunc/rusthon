global _start

section .text
_start:
    mov rax, 10
    push rax
    mov rax, 60
    pop rdi
    syscall
