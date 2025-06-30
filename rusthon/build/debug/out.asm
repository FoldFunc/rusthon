global _start
section .text
_start:
    mov rax, 3
    push rax
    mov rax, 4
    pop rbx
    xchg rax, rbx
    imul rax, rbx
    push rax
    mov rax, 3
    pop rbx
    xchg rax, rbx
    mov rdx, 0
    div rbx
    mov rdi, rax
    mov rax, 60
    syscall
