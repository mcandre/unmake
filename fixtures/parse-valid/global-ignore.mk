.POSIX:
.IGNORE:
.PHONY: clean

foo: foo.c
	gcc -o foo foo.c

clean:
	rm -rf bin
