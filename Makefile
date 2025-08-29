# Cross-compiler setup
AS = nasm
CC = i686-elf-gcc
CXX = i686-elf-g++
LD = i686-elf-ld

# Flags
ASFLAGS = -f elf32
CFLAGS = -std=gnu99 -ffreestanding -O2 -Wall -Wextra
CXXFLAGS = -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti
LDFLAGS = -ffreestanding -O2 -nostdlib -lgcc

# Source directories
KERNEL_DIR = kernel
DRIVERS_DIR = drivers
FS_DIR = fs
HAL_DIR = hal
LIBC_DIR = libc

# Output directories
BUILD_DIR = build
ISO_DIR = $(BUILD_DIR)/iso/boot/grub

# Kernel files
KERNEL_SRCS = $(wildcard $(KERNEL_DIR)/*.c) \
              $(wildcard $(KERNEL_DIR)/*/*.c) \
              $(wildcard $(HAL_DIR)/*.c) \
              $(wildcard $(DRIVERS_DIR)/*.c)
KERNEL_OBJS = $(patsubst %.c, $(BUILD_DIR)/%.o, $(KERNEL_SRCS))
KERNEL_ASM_SRCS = $(wildcard $(KERNEL_DIR)/*.asm) \
                 $(wildcard $(KERNEL_DIR)/*/*.asm)
KERNEL_ASM_OBJS = $(patsubst %.asm, $(BUILD_DIR)/%.o, $(KERNEL_ASM_SRCS))

# LibC files
LIBC_SRCS = $(wildcard $(LIBC_DIR)/*.c)
LIBC_OBJS = $(patsubst %.c, $(BUILD_DIR)/%.o, $(LIBC_SRCS))

# Final targets
KERNEL = $(BUILD_DIR)/kernel.bin
ISO = $(BUILD_DIR)/zamarine.iso

.PHONY: all clean run

all: $(ISO)

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(@D)
	$(CC) -c $< -o $@ $(CFLAGS) -Iinclude

$(BUILD_DIR)/%.o: %.asm
	@mkdir -p $(@D)
	$(AS) $(ASFLAGS) $< -o $@

$(KERNEL): $(KERNEL_OBJS) $(KERNEL_ASM_OBJS) $(LIBC_OBJS)
	$(CC) -T linker.ld -o $@ $(LDFLAGS) $^ -lgcc

$(ISO): $(KERNEL)
	@mkdir -p $(ISO_DIR)
	cp $(KERNEL) $(ISO_DIR)/
	echo 'set timeout=0' > $(ISO_DIR)/grub.cfg
	echo 'set default=0' >> $(ISO_DIR)/grub.cfg
	echo 'menuentry "Zamarine OS" {' >> $(ISO_DIR)/grub.cfg
	echo '  multiboot /boot/kernel.bin' >> $(ISO_DIR)/grub.cfg
	echo '  boot' >> $(ISO_DIR)/grub.cfg
	echo '}' >> $(ISO_DIR)/grub.cfg

	grub-mkrescue -o $@ $(BUILD_DIR)/iso

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO) -serial stdio

clean:
	rm -rf $(BUILD_DIR)
