all: main
	./main

CC=clang
CFLAGS=-g -O0

main: main.c
	$(CC) $(LDFLAGS) $(CFLAGS) -o main main.c 

clean:
	rm -f main

