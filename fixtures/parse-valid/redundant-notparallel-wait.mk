.POSIX:
.NOTPARALLEL:
.PHONY: test test-1 test-2

test: test-1 .WAIT test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
