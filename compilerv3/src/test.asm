extern malloc
extern exit
global _start

section .text
_start:
    ; Allocate space for variable table: 1000 * 8 bytes = 8000 bytes
    mov rdi, 8000
    call malloc
    mov r12, rax       ; r12 = pointer to variable table

    ; Allocate 8 bytes for variable x
    mov rdi, 8
    call malloc
    mov qword [r12 + 0], rax     ; var[0] = pointer to x
    mov qword [rax], 6           ; *x = 6
cond:
    mov rax, 10
    cmp 10, 10 
    je cond
    mov r13, qword [r12 + 0]     ; load pointer to x
    mov rdi, qword [r13]         ; load value of x
    call exit

