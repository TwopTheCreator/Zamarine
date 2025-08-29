global _start

section .data
    msg db "Hello from Zamarine Assembly app!", 10
    len equ $-msg

section .text
_start:
    mov rax, 1          ; write
    mov rdi, 1          ; stdout
    mov rsi, msg
    mov rdx, len
    syscall

    mov rax, 60         ; exit
    xor rdi, rdi
    syscall


