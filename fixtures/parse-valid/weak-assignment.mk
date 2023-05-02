.POSIX:
.PHONY: all

FRUIT=apple
FRUIT?=banana

all:
	@echo "FRUIT: $(FRUIT)"
