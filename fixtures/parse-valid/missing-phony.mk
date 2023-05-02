.POSIX:

all:
	echo "Hello World!"

lint:;

install:;

uninstall:;

publish:;

test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"

clean:
	-rm -rf bin

port: cross-compile archive

empty:;
