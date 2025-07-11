global _start

section .bss
x: resq 1        ; reserve 8 bytes for x (64-bit)

section .text
_start:
    mov rax, 10
    mov [x], rax ; store 10 in x

    ; simulate "return 69"
    mov rdi, [x]
    mov rax, 60
    syscall

