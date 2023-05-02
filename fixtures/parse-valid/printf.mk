.POSIX:
.PHONY: test-1 test-2

test-1:
	@printf "Hello World!\n"

MSG="Hello World!\n"

test-2:
	@printf $(MSG)
