
.PHONY: test

# foo is the application binary
foo: foo.c
# gcc is a conventional Linux compiler
	gcc -o foo foo.c
# -o foo emits a foo application binary
# the test target runs the test suite
test: foo
	./foo
# test is marked as phony above,
# to ensure that the target can still actually run
# in the event that a file named test is ever created

