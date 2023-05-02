.POSIX:
.PHONY: all foo bar one two
all: foo bar
foo: one two
bar: one two
foo bar one two: ; @echo $@
