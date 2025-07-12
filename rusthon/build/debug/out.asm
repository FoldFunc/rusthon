global _start
section .text
_start:
    ; … your prologue here (e.g. sub rsp…)
    mov byte rax, 'c'
    ; store var: text in global memory
    mov [text], rax
    xor rax, rax
    mov rax, 4
    ; store var: x in global memory
    mov [x], rax
    xor rax, rax
    mov rax, 3
    push rax
    mov rax, [x]
    push rax
    mov rax, 2
    pop rbx
    add rax, rbx
    pop rbx
    xchg rax, rbx
    imul rax, rbx
   ; re declare var: x in global memory
   mov [x], rax
   xor rax, rax
    mov rax, [x]
    mov rdi, rax
    mov rax, 60
    syscall

section .bss
text:    resq 1
x:    resq 1
