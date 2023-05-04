.POSIX:
.SILENT:
.PHONY: \
	all \
	foo \
	clean

all: foo

foo: foo.c
	gcc -o foo foo.c

clean:
	rm -f foo
