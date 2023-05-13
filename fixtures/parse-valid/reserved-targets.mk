.PHONY: all test .TEST .TEST-UNIT .TEST-INTEGRATION

all: test

.TEST:
	echo "Hello World!"

test: .TEST-UNIT .TEST-INTEGRATION
