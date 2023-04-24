MACRO = value1
Immed ::= $(MACRO)
DELAY = $(MACRO)
MACRO = value2

target:
	echo $(Immed) $(DELAY)
