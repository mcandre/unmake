.PHONY: all "foo"

all: "foo"

"foo": foo.c
	gcc -o "foo" foo.c
