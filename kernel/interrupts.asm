global idt_load
global isr0
global isr1
; Add more ISR declarations as needed

extern isr_handler

; Macro to push all registers and the data segment
%macro ISR_NOERRCODE 1
  global isr%1
  isr%1:
    cli
    push byte 0
    push byte %1
    jmp isr_common_stub
%endmacro

%macro ISR_ERRCODE 1
  global isr%1
  isr%1:
    cli
    push byte %1
    jmp isr_common_stub
%endmacro

; Define ISRs
ISR_NOERRCODE 0
ISR_NOERRCODE 1
; Add more ISRs as needed

; Common ISR code
isr_common_stub:
    pusha
    mov ax, ds
    push eax
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    call isr_handler
    
    pop eax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    popa
    add esp, 8
    iret

; Load IDT
idt_load:
    mov eax, [esp+4]
    lidt [eax]
    ret
