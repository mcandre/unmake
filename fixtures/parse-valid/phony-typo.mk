.POSIX:
.SILENTT: all foo test
.PHONY: all foo test

all: test

foo: foo.c
	gcc -o foo foo.c

test: foo
	./foo
