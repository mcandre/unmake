BIN=foo

$(BIN): foo.c
	gcc -o $(BIN) foo.c
