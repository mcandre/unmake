# PARSING

`unmake` follows a relatively stiff reading of the POSIX `make` standard. Our aim is to promote more portable, maintainable makefiles.

# POSIX

unmake targets POSIX 202x Issue 8, Draft 3.

https://www.opengroup.org/austin/

# CAVEATS

We do our best to catch many of the more obvious POSIX make mistakes, but some mistakes may continue to slip up nonetheless. For example:

* Accidents hiding inside macro expressions
* Accidents hiding inside command output
* Accidents hiding inside `include` chains
* Misuse of reserved target names
* Behavior during live `make` script execution
* GNU/BSD/etc. extensions beyond the POSIX standard
* Variance in make implementation adherence to the POSIX standard
* Clock and file system timestamp variance
* Files not named like `makefile` or `*.mk`
* Logic errors

make users are advised to generally familiarize with basic make usage, syntax, and semantics, in order to resolve problems faster.

# MAKEFILE SYNTAX QUIRKS

## Line endings

The legacy line endings CRLF (`\r\n`) and CR (`\r`) are out of spec. make expects LF (`\n`).

In fact, the POSIX standard specifies that text files the LF line ending, as well as a final LF End Of Line (EOL) character at the end of any non-zero-byte text file.

### Fail

We briefly list some illustrative makefile contents expected to fail parsing.

```make
all:<CRLF>
	echo "Hello World!"
```

```make
all:<CR>
	echo "Hello World!"
```

### Pass

We briefly list some illustrate makefile contents expected to pass parsing.

```make
all:<LF>
	echo "Hello World!"<LF>
```

### Mitigation

* Note that recent editions of Notepad are LF-aware, but may continue to accidentally default to CRLF when creating new files. Use a more capable text editor, such as [emacs](https://www.gnu.org/software/emacs/), [nano](https://www.nano-editor.org/), [Notepad++](https://notepad-plus-plus.org/), [vim](https://www.vim.org/), [VSCode](https://code.visualstudio.com/), etc.
* Configure [EditorConfig](https://editorconfig.org/) and your text editor, to default most any non-Windows-centric text files to LF line endings and a final EOL.
* If necessary, relaunch the text editor and resave the file.
* Reset [git](https://git-scm.com/)'s configuration (both the git-config and gitattributes systems) to the default behavior, which tends to preserve file endings as written
* Apply [tofrodos](https://www.thefreecountry.com/tofrodos/index.shtml)'s `fromdos` utility on affected files

## Rule command indentation

make generally expects rule commands to be indented with a tab (`\t`).

Command lines preceded by an escaped newline (`\\\n`), as in a command line argument continuation, may omit the tab, or indent arguments with additional tabs.

### Fail

```make
all:
echo "Hello World!"
```

```make
all:
<spaces>echo "Hello World!"
```

```make
all:
<mixed spaces and tabs>echo "Hello World!"
```

### Pass

```make
all:
<tab>echo "Hello World!"
```

### Mitigation

* Configure [EditorConfig](https://editorconfig.org/) and your text editor, to use hard tabs for indentation in makefiles. Reindent the rule declarations. Resave the file.
* Indent `*.go` files with hard tabs as well.
* Other languages are steadily converging on four spaces for indentation, like C, C++, CSS, HTML, Java, JavaScript, Python, Ruby, Rust, shell, ...
* LISP and free-form text files, may not conform to K&R style C indentation. They use variable width indentation, which EditorConfig can model as essentially a form of single space indentation.
* Consider indenting any multiline command arguments an additional tab column deeper than its parent command, for readability.

## Rule Wholeness

Rule declarations generally require at least one of the following: A prerequisite, an inline command, and/or an indented command.

Rules intentionally set to empty, should generally feature a trailing semicolon (`test:;`).

Ancient makefiles once used a convention of an empty prerequisite, in order to force make to freshly rebuild other targets. However, modern make enjoys the standard `.PHONY` special target for this.

Note that certain special targets are allowed at parse level to be declared with zero prerequisites + zero inline commands + zero indented commands: `.POSIX:`, `.IGNORE:`, `.NOTPARALLEL:`, `.PHONY:`, `.PRECIOUS:`, `.SILENT:`, `.SUFFIXES:`, and `.WAIT:`. They may or may not accept a semicolon inline command, even a blank one.

### Fail

```make
test:
```

```make
DIR: FORCE
	ls DIR

FORCE:
```

### Pass

```make
test: unit-test
```

```make
test:
	echo "Hello World!"
```

```make
test:; echo "Hello World!"
```

```make
test:;
```

```make
# test:
```

```make
<remove extraneous rules>
```

```make
.PHONY: DIR

DIR:
	ls DIR
```

### Mitigation

* Give the rule something useful to do: Introduce at least one prerequisite, indented command, and/or inline command.
* Use `.PHONY` to denote targets that should always be freshly rebuilt.
* Explicitly mark empty rules with reset notation (`<target>:;`)
* Certain special targets may be allowed to be empty. Other special targets may reject the `<target>:;` reset notation.
* Comment out temporarily empty rules
* Remove extraneous rules

## Assignment Operator Portability

Due to incompatible semantics between different make implementations for the `:=` operator, the POSIX standard discourages this operator.

### Fail

```make
M := hello
```

### Pass

```make
M = hello
```

```make
M ::= hello
```

```make
M :::= hello
```

```make
M ?= hello
```

```make
M != echo hello
```

```make
M += hello
```

### Mitigation

* Identify the desired behavior, and apply a corresponding POSIX compliant macro assignment operator.

## Incomplete macro definition

Macro definitions take the form `<name> <assignment operator> [<value>]`.

The first two tokens, `<name>`, and `<assignment operator>` are required. The `<value>` may be omitted, when declaring a blank macro.

### Fail

```make
=
```

```make
=1
```

```make
A
```

(Or the relevant POSIX make assignment operator)

### Pass

```make
A = 1
```

```make
A=
```

(Or the relevant POSIX make assignment operator)

### Mitigation

* Bind a specific macro name to some value.
* Bind a specific macro name to a blank value.

## Include line spaces and quoting

include lines use whitespace to delimit separate file paths. However, POSIX prohibits include lines from using double-quotes (`""`).

### Fail

```make
include "foo.mk"
```

### Pass

```make
include foo.mk
```

### Mitigation

* Remove double quotes from include paths
* Avoid spaces in file and directory names
* Simplify path names

### Special target isolation

When declaring rules with special targets, avoid supplying multiple targets.

Note that `.WAIT` is ignored when specified as a direct target.

### Fail

```make
.SILENT .IGNORE:
```

```make
.SILENT test: unit-test
```

```make
.WAIT test lint: unit-test
```

### Pass

```make
.SILENT:

.IGNORE:
```

```make
.SILENT: unit-test

test: unit-test
```

```make
test: unit-test .WAIT lint
```

### Mitigation

* Limit special targets to once per rule declaration.
* Avoid using the `.WAIT` special target as a direct target. Instead, use `.WAIT` as a prerequisite modifier.

### Multiline expressions

Note that escaped newlines are distinct from newline literals, for example, `\n` in the UNIX command `printf "Hello World!\n"`.

POSIX make presents two distinct semantics for multiline expressions, using escaped newlines (`\\\n`).

When an escaped newline occurs in a macro definition value, then the escaped newline is replaced with a single space.

When an escaped newline occurs in a rule command, then the escaped newline is preserved and forwarded as a part of the final multiline command to be executed. The subsequent is allowed to optionally omit the usual tab indentation. The first tab is not preserved in the final multiline command.

Note that in both cases of multiline macro definitions and multiline rule commands, whitespace sensitivity may lead to subtle processing errors.

POSIX prohibits escaped newlines in include lines.

Escaped newlines followed directly by the end of file (`<eof>`), or subsequent lines that don't make sense for the preceding line, may trigger in parse errors.

Escaped newlines featuring trailing whitespace between the backslash and the line feed, may trigger parse errors.

Escaped newlines occuring elsewhere in a makefile, may lack an official POSIX parsing behavior, and may reduce the makefile's human readability. For example, in comments or general macro expressions.

The portability risk of multiline rule commands is relatively minor. Most make implementations set the `SHELL` macro to a POSIX compatible `sh` interpreter.

### Fail

```make
include \
foo.mk
```

```make
NAMES \
= Alice\
Bob\
Charlie
```

```make
provision \
:
	apt-get install -y \
		cargo \
		rust \

clean:
	clean cargo
```

### Pass

```make
include foo.mk
```

```make
NAMES = Alice\
Bob\
Charlie
```

```make
provision:
	apt-get install -y \
		cargo \
		rust

clean:
	clean cargo
```

### Mitigation

* Configure [EditorConfig](https://editorconfig.org/) to remove most trailing whitespace.
* Avoid use of multiline instructions in makefiles in the left side of macro assignment operators, in the opening line of rule declaration blocks, in general macro expressions, and near comments.
* Handle whitespace carefully.
* Handle multiline termination carefully.
* Reserve multilines for complex macro values or rule commands, especially those expected to grow over time.
* Move packages installed via [NPM](https://www.npmjs.com/) or [pip](https://pypi.org/project/pip/), to a relevant configuration file.
* Consider moving complex rule logic to a separate makefile, script, or compiled application.
* For classic Windows environments, acquire make from [WSL](https://learn.microsoft.com/en-us/windows/wsl/install), [Chocolatey](https://chocolatey.org/), etc., which provide POSIX compatible interpreters.
* For vintage environments like MS-DOS, FreeDOS, or OS/2 Warp, move complex rule commands to a dedicated script.

## make implementation extensions

Popular make implementations like GNU gmake and BSD bmake, support syntax extensions beyond POSIX. Certain implementation-specific syntax may trigger parse errors.

### Fail

```make
.for i in 1 2 3
test-double-${i}:
	@echo "2 * ${i}" | bc
.endfor
```

(Specifically, the `.for` and `.endfor` lines, though the exact parse error indication may vary.)

### Mitigation

* Familiarize with make implementation-specific syntax.
* Rename GNU-specific makefiles to like `GNUmakefile`, for clarity.
* Consider moving complex logic to a separate makefile, script, or compiled application.
