global _start
section .text
_start:
    ; … your prologue here (e.g. sub rsp…)
    mov rax, 10
    push rax
    mov rax, 10
    pop rbx
    add rax, rbx
    ; store var: x in global memory
    mov [x], rax
    xor rax, rax
    mov rax, [x]
    mov rdi, rax
    mov rax, 60
    syscall

section .bss
x:    resq 1
