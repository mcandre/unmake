# Everything all at once
.POSIX:
.DEFAULT:;
.IGNORE:
.NOTPARALLEL:
.PHONY: all test clean
.PRECIOUS:
.SCCS_GET:;
.SILENT:
.SUFFIXES:
.WAIT:

all: test

test: foo
	./foo

foo: foo.c
	gcc -o foo foo.c

clean:
	-rm foo
