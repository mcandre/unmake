.POSIX:
.PHONY: all

CLIENT=\curl

all:
	$(CLIENT) https://www.google.com/index.html
