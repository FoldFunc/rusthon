global _start
_start:
    mov rax, 60
    mov rdi, 1 + 2 + (3 * 4) ; I can do that
    syscall
