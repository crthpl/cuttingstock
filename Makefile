all: main
	./main

main: main.c
	$(CC) $(LDFLAGS) $(CFLAGS) -o main main.c 

clean:
	rm -f main

