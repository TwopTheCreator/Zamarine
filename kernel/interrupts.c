#include <stdint.h>
#include <stddef.h>

struct idt_entry {
    uint16_t base_lo;
    uint16_t sel;
    uint8_t always0;
    uint8_t flags;
    uint16_t base_hi;
} __attribute__((packed));

struct idt_ptr {
    uint16_t limit;
    uint32_t base;
} __attribute__((packed));

struct idt_entry idt[256];
struct idt_ptr idtp;

extern void idt_load();

extern void isr0();
extern void isr1();
// ... Add more ISR declarations as needed

void idt_set_gate(uint8_t num, uint32_t base, uint16_t sel, uint8_t flags) {
    idt[num].base_lo = (base & 0xFFFF);
    idt[num].base_hi = (base >> 16) & 0xFFFF;
    idt[num].sel = sel;
    idt[num].always0 = 0;
    idt[num].flags = flags | 0x60;
}

void idt_install() {
    idtp.limit = (sizeof(struct idt_entry) * 256) - 1;
    idtp.base = (uint32_t)&idt;

    // Initialize the IDT to zeros
    unsigned char *idt_ptr = (unsigned char *)&idt;
    for (int i = 0; i < sizeof(struct idt_entry) * 256; i++) {
        idt_ptr[i] = 0;
    }

    // Set up the IDT entries for ISRs
    idt_set_gate(0, (unsigned)isr0, 0x08, 0x8E);
    idt_set_gate(1, (unsigned)isr1, 0x08, 0x8E);
    // Add more IDT entries as needed

    // Load the IDT
    idt_load();
}

// Default interrupt handler
void isr_handler(struct regs *r) {
    // Handle the interrupt here
    terminal_writestring("Interrupt received!\n");
}
