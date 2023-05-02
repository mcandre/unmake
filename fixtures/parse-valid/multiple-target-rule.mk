.POSIX:
all: foo bar
foo: one two
bar: one two
foo bar one two: ; @echo $@
