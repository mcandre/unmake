.POSIX:

foo: foo.c
	#build foo
	gcc -o foo foo.c

foo: foo.c
	@#gcc -o foo foo.c

foo: foo.c
	-#gcc -o foo foo.c

foo: foo.c
	+#gcc -o foo foo.c

foo: foo.c
	gcc \
#output file \
		-o foo \
		foo.c
