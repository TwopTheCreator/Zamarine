#include <kernel.h>
#include <stdint.h>
#include <stddef.h>

#define PMM_BLOCK_SIZE 4096
#define PMM_BLOCKS_PER_BYTE 8

static uint32_t memory_size = 0;
static uint32_t used_blocks = 0;
static uint32_t max_blocks = 0;
static uint32_t* memory_map = 0;

static void pmm_mmap_set(int bit) {
    memory_map[bit / 32] |= (1 << (bit % 32));
}

static void pmm_mmap_unset(int bit) {
    memory_map[bit / 32] &= ~(1 << (bit % 32));
}

static int pmm_mmap_test(int bit) {
    return memory_map[bit / 32] & (1 << (bit % 32));
}

void pmm_init(size_t mem_size, uint32_t* bitmap) {
    memory_size = mem_size;
    memory_map = bitmap;
    max_blocks = (memory_size * 1024) / PMM_BLOCK_SIZE;
    used_blocks = max_blocks;
    
    // By default, mark all memory as used
    memset(memory_map, 0xff, max_blocks / PMM_BLOCKS_PER_BYTE);
}

void pmm_init_region(uint32_t base, size_t size) {
    int align = base / PMM_BLOCK_SIZE;
    int blocks = size / PMM_BLOCK_SIZE;
    
    for (; blocks > 0; blocks--) {
        pmm_mmap_unset(align++);
        used_blocks--;
    }
    
    // The first block is always set to prevent returning NULL
    pmm_mmap_set(0);
}

void pmm_deinit_region(uint32_t base, size_t size) {
    int align = base / PMM_BLOCK_SIZE;
    int blocks = size / PMM_BLOCK_SIZE;
    
    for (; blocks > 0; blocks--) {
        pmm_mmap_set(align++);
        used_blocks++;
    }
}

void* pmm_alloc_block() {
    if (used_blocks >= max_blocks)
        return 0; // Out of memory
        
    int frame = 0;
    while (frame < max_blocks) {
        if (memory_map[frame / 32] != 0xFFFFFFFF) {
            for (int bit = 0; bit < 32; bit++) {
                int bit_test = 1 << bit;
                if (!(memory_map[frame / 32] & bit_test)) {
                    memory_map[frame / 32] |= bit_test;
                    used_blocks++;
                    return (void*)((frame * 32 + bit) * PMM_BLOCK_SIZE);
                }
            }
        }
        frame++;
    }
    return 0; // Out of memory
}

void pmm_free_block(void* p) {
    uint32_t addr = (uint32_t)p;
    uint32_t frame = addr / PMM_BLOCK_SIZE;
    
    if (frame >= max_blocks)
        return;
        
    if (!pmm_mmap_test(frame))
        return;
        
    pmm_mmap_unset(frame);
    used_blocks--;
}

void* pmm_alloc_blocks(size_t size) {
    if (size == 0)
        return 0;
        
    if (used_blocks + size > max_blocks)
        return 0; // Not enough memory
        
    uint32_t frame = 0;
    uint32_t needed_blocks = (size + PMM_BLOCK_SIZE - 1) / PMM_BLOCK_SIZE;
    
    while (frame < max_blocks) {
        if (memory_map[frame / 32] != 0xFFFFFFFF) {
            for (int bit = 0; bit < 32; bit++) {
                if (!(memory_map[frame / 32] & (1 << bit))) {
                    int start_frame = frame * 32 + bit;
                    int free_blocks = 0;
                    
                    for (uint32_t i = 0; i < needed_blocks; i++) {
                        if (start_frame + i >= max_blocks)
                            return 0; // Not enough contiguous blocks
                            
                        if (pmm_mmap_test(start_frame + i))
                            break;
                            
                        free_blocks++;
                    }
                    
                    if (free_blocks >= needed_blocks) {
                        for (uint32_t i = 0; i < needed_blocks; i++)
                            pmm_mmap_set(start_frame + i);
                            
                        used_blocks += needed_blocks;
                        return (void*)(start_frame * PMM_BLOCK_SIZE);
                    }
                }
            }
        }
        frame++;
    }
    
    return 0; // No suitable block range found
}

void pmm_free_blocks(void* p, size_t size) {
    if (p == 0 || size == 0)
        return;
        
    uint32_t addr = (uint32_t)p;
    uint32_t start_frame = addr / PMM_BLOCK_SIZE;
    uint32_t blocks = (size + PMM_BLOCK_SIZE - 1) / PMM_BLOCK_SIZE;
    
    for (uint32_t i = 0; i < blocks; i++) {
        if (pmm_mmap_test(start_frame + i)) {
            pmm_mmap_unset(start_frame + i);
            used_blocks--;
        }
    }
}

size_t pmm_get_memory_size() {
    return memory_size * 1024; // Return size in bytes
}

uint32_t pmm_get_used_blocks() {
    return used_blocks;
}

uint32_t pmm_get_free_blocks() {
    return max_blocks - used_blocks;
}
