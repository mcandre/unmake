.POSIX:
.PHONY: test

test: foo.c
	gcc -o foo foo.c
	./foo
