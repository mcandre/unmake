.POSIX:
.PHONY: all

FIRST=Alice 
LAST=Liddell
FULL_NAME=$(FIRST)$(LAST)

all: 
	@echo "$(FULL_NAME)" 
