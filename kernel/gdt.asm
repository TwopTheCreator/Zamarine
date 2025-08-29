global gdt_flush
extern gp

gdt_flush:
    lgdt [gp]
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x08:flush2
flush2:
    ret

; GDT structure
global _gdt
global _gdt_end

section .data
; GDT entries
_gdt:
    ; Null descriptor
    dd 0
    dd 0

    ; Code segment descriptor
    dw 0xFFFF       ; Limit (bits 0-15)
    dw 0x0000       ; Base (bits 0-15)
    db 0x00         ; Base (bits 16-23)
    db 0x9A         ; Access byte (present, ring 0, code segment, executable, readable)
    db 0xCF         ; Flags (granularity, 32-bit) + Limit (bits 16-19)
    db 0x00         ; Base (bits 24-31)

    ; Data segment descriptor
    dw 0xFFFF       ; Limit (bits 0-15)
    dw 0x0000       ; Base (bits 0-15)
    db 0x00         ; Base (bits 16-23)
    db 0x92         ; Access byte (present, ring 0, data segment, writable)
    db 0xCF         ; Flags (granularity, 32-bit) + Limit (bits 16-19)
    db 0x00         ; Base (bits 24-31)

_gdt_end:

; GDT pointer
gp:
    dw _gdt_end - _gdt - 1  ; GDT limit
    dd _gdt                 ; GDT base address
