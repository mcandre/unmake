.c.o\
:
	echo "TBD"

a-2.txt\
b-2.txt\
c-2.txt\
\
:\
\
a-1.txt\
a-2.txt\
\
a-3.txt
	cat a-1.txt >a-2.txt
	cat b-1.txt >b-2.txt
	cat c-1.txt >c-2.txt

greet\
:\
;\
echo "Hello World!"

.POSIX\
:
.DEFAULT\
:\
;
.IGNORE\
:
.NOTPARALLEL\
:
.PHONY\
:\
all test clean
.PRECIOUS\
:
.SCCS_GET\
:\
;
.SILENT\
:
.SUFFIXES\
:
.WAIT\
:

all\
:\
test

test\
:\
foo
	./foo

foo\
:\
foo.c
	gcc -o foo foo.c

clean\
:
	-rm foo
