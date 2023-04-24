all: foo bar
foo: one .WAIT two
bar: one two
foo bar one two: ; @echo $@
