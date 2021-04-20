
CC=clang
CFLAGS=-g

main: foo.o math.o
	$(CC) $(CFLAGS) -o $@ $^
	@echo done

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $^

clean:
	rm *.o main

