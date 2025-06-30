global main
section .text
main:
    sub rsp, 64

    mov rax, 10
    mov [rsp - 8], rax

    mov rbx, [rsp - 8]
    mov rax, 60
    xor rdi, rdi
    syscall

