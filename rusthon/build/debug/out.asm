global _start
section .text
_start:
mov rdi, 10
mov rax, 60
syscall
