.POSIX:
.PHONY: all

all: foo%

foo%: foo.c
	gcc -o foo% foo.c
