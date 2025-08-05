global _start
section .text
_start:
    mov rax, 10
    mov [x], rax
    xor rax, rax
    mov rdi, [x]
    mov rax, 60
    syscall

section .bss
x: resq 1
