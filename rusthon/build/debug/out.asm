global _start
section .text
_start:
    sub rsp, 64
    mov rax, 1
    mov [vec0_addr + 8*0], rax
    mov rax, 2
    mov [vec0_addr + 8*1], rax
    mov rax, 3
    mov [vec0_addr + 8*2], rax
    lea rax, [vec0_addr]
    mov [list], rax
    mov byte rax, 'c'
    mov [text], rax
    mov rax, 10
    mov [x], rax
    mov rax, [x]
    mov rdi, rax
    mov rax, 60
    syscall

section .bss
text:    resq 1
list:    resq 1
list0_addr:    resq 3
x:    resq 1
vec0_addr: resq 3
