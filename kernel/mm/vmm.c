#include <kernel.h>
#include <stdint.h>
#include <stddef.h>
#include <string.h>

// Page directory and table entry flags
#define PTE_PRESENT 0x1
#define PTE_WRITABLE 0x2
#define PTE_USER 0x4
#define PTE_WRITETHROUGH 0x8
#define PTE_CACHE_DISABLED 0x10
#define PTE_ACCESSED 0x20
#define PTE_DIRTY 0x40
#define PTE_PAT 0x80
#define PTE_GLOBAL 0x100
#define PTE_FRAME 0xFFFFF000

// Page table and directory indexes
#define PT_INDEX(vaddr) (((uint32_t)vaddr >> 12) & 0x3FF)
#define PD_INDEX(vaddr) (((uint32_t)vaddr >> 22) & 0x3FF)

// Page directory and table types
typedef uint32_t pd_entry_t;
typedef uint32_t pt_entry_t;

// The kernel's page directory
static pd_entry_t* kernel_directory = 0;
// The current page directory
static pd_entry_t* current_directory = 0;

// The kernel's heap
static uint32_t* frames;
static uint32_t nframes;

extern uint32_t placement_address;

// Defined in pmm.c
extern void pmm_free_block(void*);
extern void* pmm_alloc_block();

// Internal function to set a frame
static void set_frame(uint32_t frame_addr) {
    uint32_t frame = frame_addr / 0x1000;
    uint32_t idx = frame / 32;
    uint32_t off = frame % 32;
    frames[idx] |= (1 << off);
}

// Internal function to clear a frame
static void clear_frame(uint32_t frame_addr) {
    uint32_t frame = frame_addr / 0x1000;
    uint32_t idx = frame / 32;
    uint32_t off = frame % 32;
    frames[idx] &= ~(1 << off);
}

// Internal function to test if a frame is set
static uint32_t test_frame(uint32_t frame_addr) {
    uint32_t frame = frame_addr / 0x1000;
    uint32_t idx = frame / 32;
    uint32_t off = frame % 32;
    return (frames[idx] & (1 << off));
}

// Internal function to find the first free frame
static uint32_t first_frame() {
    for (uint32_t i = 0; i < nframes / 32; i++) {
        if (frames[i] != 0xFFFFFFFF) {
            for (uint32_t j = 0; j < 32; j++) {
                if (!(frames[i] & (1 << j))) {
                    return i * 32 + j;
                }
            }
        }
    }
    return -1; // No free frames
}

// Allocate a frame for a page
void alloc_frame(pt_entry_t* page, int is_kernel, int is_writeable) {
    if (page->frame != 0) {
        return; // Frame was already allocated
    } else {
        uint32_t idx = first_frame();
        if (idx == (uint32_t)-1) {
            // PANIC! No free frames
            panic("No free frames!");
        }
        set_frame(idx * 0x1000);
        page->frame = idx;
        page->present = 1;
        page->rw = is_writeable ? 1 : 0;
        page->user = is_kernel ? 0 : 1;
    }
}

// Free a frame from a page
void free_frame(pt_entry_t* page) {
    if (page->frame) {
        clear_frame(page->frame);
        page->frame = 0;
    }
}

// Initialize the virtual memory manager
void vmm_init() {
    // The size of physical memory
    uint32_t mem_end_page = 0x1000000; // 16MB for now
    
    nframes = mem_end_page / 0x1000;
    frames = (uint32_t*)kmalloc(nframes / 8);
    memset(frames, 0, nframes / 8);
    
    // Create a page directory
    kernel_directory = (pd_entry_t*)kmalloc_a(sizeof(pd_entry_t) * 1024);
    current_directory = kernel_directory;
    
    // Map some memory for the kernel heap
    for (uint32_t i = KERNEL_VIRTUAL_BASE; i < KERNEL_VIRTUAL_BASE + 0x400000; i += 0x1000) {
        get_page(i, 1, kernel_directory);
    }
    
    // Register interrupt handlers for page faults
    register_interrupt_handler(14, page_fault);
    
    // Enable paging
    enable_paging(kernel_directory);
}

// Handle a page fault
void page_fault(registers_t* regs) {
    uint32_t faulting_address;
    __asm__ volatile("mov %%cr2, %0" : "=r" (faulting_address));
    
    int present = !(regs->err_code & 0x1);
    int rw = regs->err_code & 0x2;
    int us = regs->err_code & 0x4;
    int reserved = regs->err_code & 0x8;
    int id = regs->err_code & 0x10;
    
    // Output an error message
    terminal_writestring("Page fault! (");
    if (present) terminal_writestring("present ");
    if (rw) terminal_writestring("read-only ");
    if (us) terminal_writestring("user-mode ");
    if (reserved) terminal_writestring("reserved ");
    terminal_writestring(") at 0x");
    
    // Convert the address to a string and print it
    char str[32] = {0};
    itoa(faulting_address, str, 16);
    terminal_writestring(str);
    terminal_putchar('\n');
    
    panic("Page fault");
}

// Enable paging
void enable_paging(pd_entry_t* page_directory) {
    // Write the page directory address to CR3
    asm volatile("mov %0, %%cr3" :: "r"(page_directory));
    
    // Set the paging bit in CR0
    uint32_t cr0;
    asm volatile("mov %%cr0, %0": "=r"(cr0));
    cr0 |= 0x80000000; // Set PG bit
    asm volatile("mov %0, %%cr0" :: "r"(cr0));
}

// Get the page from a virtual address
pt_entry_t* get_page(uint32_t address, int make, pd_entry_t* dir) {
    // Turn the address into an index
    address /= 0x1000;
    // Find the page table containing this address
    uint32_t table_idx = address / 1024;
    
    if (dir[table_idx]) { // If this table is already assigned
        return &((pt_entry_t*)(dir[table_idx] & 0xFFFFF000))[address % 1024];
    } else if (make) {
        uint32_t tmp;
        dir[table_idx] = (uint32_t)kmalloc_ap(sizeof(pt_entry_t) * 1024, &tmp);
        dir[table_idx] |= 0x7; // Present, R/W, User
        return &((pt_entry_t*)(dir[table_idx] & 0xFFFFF000))[address % 1024];
    } else {
        return 0;
    }
}
