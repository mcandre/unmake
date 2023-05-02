.POSIX:

FRUIT=apple
FRUIT?=banana

all:
	@echo "FRUIT: $(FRUIT)"
