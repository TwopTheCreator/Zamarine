section .text
global fast_memcmp

fast_memcmp:
    push rdi
    push rsi
    push rbx
    
    mov rdi, rcx    ; ptr1
    mov rsi, rdx    ; ptr2
    mov rcx, r8     ; length
    
    test rcx, rcx
    jz .equal
    
    mov rax, -1
    repz cmpsb
    jne .not_equal
    
.equal:
    xor eax, eax
    jmp .done
    
.not_equal:
    seta al
    movzx eax, al
    lea rax, [rax*2 - 1]
    
.done:
    pop rbx
    pop rsi
    pop rdi
    ret
