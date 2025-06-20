.POSIX:
.SILENT:
ALLTARGETS!=ls -a *
.PHONY: $(ALLTARGETS)

all: welcome

welcome:
	echo "Hello World!"
