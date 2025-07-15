.POSIX:
.PHONY: all

CLIENT := curl --version

all:
	$(CLIENT)
