CC=gcc
CFLAGS=-O2 -pipe -Wall
PREFIX=/usr/local

all: hello-c

hello-c: main.c
	$(CC) $(CFLAGS) -o $@ $<

install: hello-c
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp hello-c $(DESTDIR)$(PREFIX)/bin/

clean:
	rm -f hello-c


