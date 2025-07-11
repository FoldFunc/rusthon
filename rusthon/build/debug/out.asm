global _start
section .text
_start:
    ; … your prologue here (e.g. sub rsp…)
    mov rax, 10
    ; store var: y in global memory
    mov [y], rax
    xor rax, rax
    mov rax, [x]
    mov rdi, rax
    mov rax, 60
    syscall

section .bss
y:    resq 1
