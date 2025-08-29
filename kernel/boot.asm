section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:

section .text
bits 32
global _start
extern kernel_main
_start:
    cli
    mov esp, stack_top
    push eax
    push ebx
    call kernel_main
    hlt

section .bss
align 16
stack_bottom:
    resb 16384 ; 16 KiB for stack
stack_top:
