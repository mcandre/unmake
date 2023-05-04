.POSIX:
.PHONY: all

all: foo

foo:
	 gcc -o foo foo.c

foo:
	@+- gcc -o foo foo.c

foo:
	gcc \
		-o \
		foo \
		foo.c
