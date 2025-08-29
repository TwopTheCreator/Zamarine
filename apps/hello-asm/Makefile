ASM=nasm
LD=ld
ASFLAGS=-f elf64
LDFLAGS=-static
PREFIX=/usr/local

all: hello-asm

hello-asm: main.o
	$(LD) $(LDFLAGS) -o $@ $<

main.o: main.asm
	$(ASM) $(ASFLAGS) -o $@ $<

install: hello-asm
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp hello-asm $(DESTDIR)$(PREFIX)/bin/

clean:
	rm -f hello-asm main.o


