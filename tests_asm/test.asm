global main
extern printf
section .data
	fmt db "Hello", 10, 0
section .text
main:
	mov rdi, fmt
	xor rax, rax
	call printf
	mov rdi, 60
	mov rax, 60
	syscall
