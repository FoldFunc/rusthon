global _start

section .bss
vec: resq 2              ; Reserve space for 2 quadwords

section .text
_start:
    mov qword [vec], 10          ; vec[0] = 10
    mov qword [vec + 8*1], 20    ; vec[1] = 20

    mov rdi, [vec + 8*1]         ; Load vec[1] = 20 into rdi

    mov rax, 60                  ; syscall number for exit
    syscall

