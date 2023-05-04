# WARNINGS

`unmake` offers various checks to optimize your makefiles.

Note that `unmake` does not evaluate makefiles, and therefore ignores quirks arising from macro expansions.

Most warnings feature line numbers with an approximate location of the issue in the makefile.

# General

## MISSING_FINAL_EOL

When a text file suddenly reaches End Of File (EOF) without a Line Feed (LF), then the file is said to not feature a final End Of Line (EOL).

UNIX text files expect each line to terminate in a final End Of Line, including the last line.  Omitting a final EOF can cause subtle text processing errors.

### Fail

```make
PKG = curl<EOF>
```

### Pass

```make
PKG = curl<LF>
<EOF>
```

### Mitigation

* Configure [EditorConfig](https://editorconfig.org/) and text editors to apply a final EOL.

## PHONY_TARGET

> Prerequisites of this special target are targets themselves; these targets (known as phony targets) shall be considered always out-of-date when the make utility begins executing. If a phony target’s commands are executed, that phony target shall then be considered up-to-date until the execution of make completes. Subsequent occurrences of .PHONY shall also apply these rules to the additional targets. A .PHONY special target with no prerequisites shall be ignored. If the -t option is specified, phony targets shall not be touched. Phony targets shall not be removed if make receives one of the asynchronous events explicitly described in the ASYNCHRONOUS EVENTS section.

--POSIX 202x Issue 8/D3

Briefly, make assumes that most rule targets are actual filenames. However, conventional targets named `all`, `lint`, `install`, `uninstall`, `publish`, `test*`, or `clean*`, are usually not actual filenames. These are logical targets.

When make is requested to perform these logical, top-level targets, then make needs to know not to apply the usual file-based caching. The way to do this is by declaring `.PHONY:` special rules, whose prerequisites are your logical targets.

You may write logical target declarations as whitespace delimited prerequisites in a single `.PHONY:` rule, or distribute logical target declarations among multiple `.PHONY:` rules.

As well, aggregate targets like `port: cross-compile archive`, that do not have any commands, are usually not actual filenames themselves. Aggregate, commandless targets are also logical targets. Which means that they should also have an entry as a prerequisite in a `.PHONY:` special rule.

Due to the variance in artifact names, `unmake` cannot automate checking for all possible targets deserving `.PHONY` declarations. Neither `make` nor `unmake` knows this application-specific information. The makefile maintainer should supply this information, and configure any needed `.PHONY` declarations accordingly.

### Fail

```make
all:
	echo "Hello World!"
```

```make
test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

```make
clean:
	-rm -rf bin
```

```make
empty:;
```

```make
port: cross-compile archive
```

### Pass

```make
.PHONY: all

all:
	echo "Hello World!"
```

```make
.PHONY: test test-1 test-2

test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

```make
.PHONY: clean

clean:
	-rm -rf bin
```

```make
.PHONY: empty
empty:;
```

```make
.PHONY: port

port: cross-compile archive
```

If `cross-compile` and `archive` are also logical targets, then they should be declared `.PHONY` as well.

### Mitigation

* Avoid using make build artifacts named `all`, `test*`, or `clean*`.
* Declare any targets named `all`, `lint`, `install`, `uninstall`, `publish`, `test*`, or `clean*` as `.PHONY`
* Declare command-less rule targets as `.PHONY`
* Note that POSIX usually requires a semicolon (`;`) when declaring rules without commands.
* Note that special targets like `.NOTPARALLEL`, `.PHONY`, `.POSIX`, `.WAIT`, etc., should not themselves be declared as `.PHONY`

## MAKEFILE_PRECEDENCE

> By default, the following files shall be tried in sequence: ./makefile and ./Makefile.

--POSIX 202x Issue 8/D3

Using the lowercase filename `makefile` loads slightly faster than the capitalized filename `Makefile`. The lowercase naming reduces strain on filesystem requests.

### Fail

Makefile:

```make
PKG = curl
```

### Pass

makefile:

```make
PKG = curl
```

### Mitigation

* Rename `Makefile` to `makefile`

## CURDIR_ASSIGNMENT_NOP

> The CURDIR environment variable shall not affect the value of the CURDIR macro unless the -e option is specified. If the -e option is not specified, there is a CURDIR environment variable set, and its value is different from the CURDIR macro value, the environment variable value shall be set to the macro value. If CURDIR is defined in the makefile, present in the MAKEFLAGS environment variable, or specified on the command line, it shall replace the original value of the CURDIR macro in accordance with the logical order described above, but shall not cause make to change its current working directory.

Assignment to `CURDIR` does not actually change the current working directory of the make execution.

### Fail

```make
CURDIR = build
```

### Mitigation

* Avoid assigning the `CURDIR` macro
* Note that some commands offer a built-in way to adjust the current directory, e.g. `tar -C <dir>`
* Promote complex logic to a dedicated script

## WD_NOP

make often resets the working directory across successive commands, and across successive rules. Common commands for changing directories, such as `cd`, `pushd`, and `popd`, may not have the desired effect.

Furthermore, `push` and `popd` are GNU bash extensions to the POSIX sh interpreter standard, and are likely to fail on other machines.

### Fail

```make
all:
	cd foo
```

```make
all:
	pushd foo
```

```make
all:
	popd
```

### Mitigation

* Avoid running makefile commands beginning with `cd`, `pushd`, or `popd`
* Reduce use of shell implementation-specific commands in makefiles
* Note that some commands offer a built-in way to adjust the current directory, e.g. `tar -C <dir>`
* Promote complex logic to a dedicated script

## WAIT_NOP

> When .WAIT appears as a target, it shall have no effect.

`.WAIT` is intended for use as a pseudo-prerequisite marker, in order to customize synchronization logic. `.WAIT` behaves as a useless no operation (NOP) when written as a target.

### Fail

```make
.WAIT:

test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

### Pass

```make
test: test-1 .WAIT test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

```make
test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

### Mitigation

* Use `.WAIT` as an optional pseudo-prerequisite syncronization marker
* Avoid declaring `.WAIT` as a target.

## PHONY_NOP

> A .PHONY special target with no prerequisites shall be ignored.

`.PHONY` with no prerequisites behaves as a useless no operation (NOP). When using the special target `.PHONY` rule, specify at least one prerequisite.

### Fail

```make
.PHONY:

foo: foo.c
	gcc -o foo foo.c
```

### Pass

```make
foo: foo.c
	gcc -o foo foo.c

clean:
	rm -rf bin
```

```make
.PHONY: clean

foo: foo.c
	gcc -o foo foo.c

clean:
	rm -rf bin
```

### Mitigation

* Use `.WAIT` as an optional pseudo-prerequisite syncronization marker
* Avoid declaring `.WAIT` as a target.

## REDUNDANT_NOTPARALLEL_WAIT

The `.WAIT` pseudo-prerequisite disables asynchronous processing between prerequisites of a specific rule.

`.NOTPARALLEL:` disables asyncronous processing for all prerequisites in all rules.

Using both of these special targets simultaneously is unnecessary.

### Fail

```make
.NOTPARALLEL:

test: test-1 .WAIT test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

### Pass

```make
test: test-1 .WAIT test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

```make
.NOTPARALLEL:

test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

```make
test: test-1 test-2

test-1:
	echo "Hello World!"

test-2:
	echo "Hi World!"
```

### Mitigation

* Avoid using `.NOTPARALLEL:` with `.WAIT` redundantly.
* Redundancy of `.WAIT` with `.NOTPARALLEL` is best avoided.

## REDUNDANT_SILENT_AT

At (`@`) elides an individual command from make output. This is useful for reducing log noise.

The `.SILENT` special target also elides commands from make output. If the special rule `.SILENT:` is declared with no prerequisites, then all make commands globally are silenced. If the special rule `.SILENT:` is declared with prerequisite targets, then all commands for those specific targets are silenced.

Using both of these simultaneously is unnecessary.

### Fail

```make
.SILENT:

lint:
	@unmake .
```

```make
.SILENT: lint

lint:
	@unmake .
```

### Pass

```make
.SILENT:

lint:
	unmake .
```

```make
lint:
	@unmake .
```

```make
lint:
	unmake .
```

### Mitigation

* Avoid using at (`@`) with `.SILENT` redundantly.
* Redundancy of `.SILENT` with at (`@`) is best avoided.

## REDUNDANT_IGNORE_MINUS

Hyphen-minus (`-`) continues makefile execution past soft failure exit codes of an individual command. This is useful for implementing cleanup tasks and other idempotent tasks.

The `.IGNORE` special target also continues makefile execution past soft failures. If the special rule `.SILENT:` is declared with prerequisite targets, then exit codes for commands for those specific targets are ignored. However, declaring `.IGNORE:` with no prerequisites is likely to cause subtle build problems.

Using both `-` and `.IGNORE` simultaneously is unnecessary.

If the special rule `.IGNORE:` is declared with no prerequisites, then exit codes of all make commands globally are ignored. Due to more severe issues with `.IGNORE:` declared with no prerequisites, detailed in the `GLOBAL_IGNORE` policy, the `REDUNDANT_IGNORE_MINUS` policy does not provide an automatic check for redundant `-` with a global `.IGNORE:` declaration.

### Fail

```make
.IGNORE:

clean:
	-rm -rf bin
```

```make
.IGNORE: clean

clean:
	-rm -rf bin
```

### Pass

```make
IGNORE: clean

clean:
	rm -rf bin
```

```make
clean:
	-rm -rf bin
```

```make
clean:
	rm -rf bin
```

### Mitigation

* Note that `.IGNORE:` declared with no prerequisites is likely to cause subtle build problems.
* Avoid using hyphen-minus (`-`) with `.IGNORE` redundantly.
* Redundancy of `.IGNORE` with hyphen-minus (`-`) is best avoided.

## GLOBAL_IGNORE

When the special target rule `.IGNORE:` is declared with no prerequisites, then make ignores exit codes for all make commands, for all rules. This is hazardous, and tends to invite file corruption.

Caution: Avoid using `.IGNORE:` this way. When using the special target `.IGNORE` rule, declare at least one prerequisite.

### Fail

```make
.IGNORE:

foo: foo.c
	gcc -o foo foo.c

clean:
	rm -f foo
```

### Pass

```make
.IGNORE: clean

foo: foo.c
	gcc -o foo foo.c

clean:
	rm -f foo
```

```make
foo: foo.c
	gcc -o foo foo.c

clean:
	-rm -f foo
```

```make
foo: foo.c
	gcc -o foo foo.c

clean:
	rm -f foo
```

### Mitigation

* Avoid using `.IGNORE:` without at least one prerequisite.
* Optionally, apply hyphen-minus (`-`) to individual commands.

## SIMPLIFY_AT / SIMPLIFY_MINUS

Using at (`@`) or hyphen-minus (`-`) command prefixes for several individual commands in a rule can be simplified to a `.SILENT` or `.IGNORE` declaration respectively.

Due to flexibility needs, this warning emits automatically for rules with at least two or more commands, where all of the commands feature the same at (`@`) or hyphen-minus (`-`) prefix. Rules with zero or one command, and rules with mixed command prefixes, may not trigger this warning.

We generally recommend using `.SILENT` / `.IGNORE` over individual at (`@`) / hyphen-minus (`-`).

### Fail

```make
welcome:
	-echo foo
    -echo bar
    -echo baz
```

```make
welcome:
	@echo foo
    @echo bar
    @echo baz
```

### Pass

```make
.IGNORE: welcome

welcome:
	echo foo
    echo bar
    echo baz
```

```make
.SILENT: welcome

welcome:
	echo foo
    echo bar
    echo baz
```

```make
.SILENT:

welcome:
	echo foo
    echo bar
    echo baz
```
### Mitigation

* Use `.SILENT` / `.IGNORE` targets rather than individual at (`@`) / hyphen-minus (`-`) targets.
* Note that `.IGNORE` may have poor behavior without at least one prerequisite.

## IMPLEMENTATION_DEFINED_TARGET

> The interpretation of targets containing the characters '%' and '"' is implementation-defined.

POSIX make has no portable semantic for percent signs (`%`) or double-quotes (`"`) in targets or prerequisites. Using these can vendor lock a makefile onto a specific make implementation, and/or trigger build failures.

### Fail

```make
all: foo%

foo%: foo.c
	gcc -o foo% foo.c
```

```make
all: "foo"

"foo": foo.c
	gcc -o "foo" foo.c
```

### Mitigation

* Avoid percents (`%`) and double-quotes (`"`), in targets and prerequisites.

## COMMAND_COMMENT

When a rule command contains a sharp (`#`), then make forwards the comment to the shell interpreter. This can cause the command to fail in multiline commands. This can cause the command to fail in certain shell interpreters. This increases log noise.

### Fail

```make
foo: foo.c
	#build foo
	gcc -o foo foo.c
```

```make
foo: foo.c
	@#gcc -o foo foo.c
```

```make
foo: foo.c
	-#gcc -o foo foo.c
```

```make
foo: foo.c
	+#gcc -o foo foo.c
```

```make
foo: foo.c
	gcc \
#output file \
		-o foo \
		foo.c
```

### Pass

```make
foo: foo.c
#build foo
	gcc -o foo foo.c
```

```make
#build foo
foo: foo.c
	gcc -o foo foo.c
```

```make
#output file
foo: foo.c
	gcc \
		-o foo \
		foo.c
```

```make
foo: foo.c
	gcc -o foo foo.c
```

```make
#foo: foo.c
#	gcc -o foo foo.c
```

```make
<remove rule>
```

### Mitigation

* Move comments up above multiline commands.
* Move comments to the leftmost column, fully *de*dented.
* Consider removing extraneous lines.

## REPEATED_COMMAND_PREFIX

Supplying the same command prefix multiple times is wasteful.

### Fail

```make
test:
	@@+-+--echo "Hello World!"
```

### Pass

```make
test:
	@+-echo "Hello World!"
```

```make
test:
	echo "Hello World!"
```

(Any combination of `@`, `+`, and `-` is fine as long as none of the prefix types are duplicated.)

### Mitigation

* Remove redundant code.
* Code that is redundant should be removed.

## BLANK_COMMAND

Rule commands consisting of nothing more than at (`@`), plus (`+`), minus (`-`) prefixes, and/or whitespace, can produce spurious results when the essentially empty command is executed. Without any prefixes, blank commands are likely to trigger parse errors.

Blank commands are distinct from blank lines, which normally act as comments.

Blank commands are distinct from rules that are reset to have *no* commands.

### Fail

```make
test:
	@+-
```

### Pass

```make
test:
	@+-echo "Hello World!"
```

```make
test:
	@+-echo "Hello World!"
```

```make
test:;
```

```make
#test:
```

```make
<rule removed>
```

### Mitigation

* Give the command something useful to do.
* Remove extraneous code.

## NO_RULES

make generally expects a makefile to define at least one (non-special) rule to provide some action on when running `make`. Excepting include files like `sys.mk` or `*.include.mk`.

### Fail

makefile:

```make
.POSIX:
PKG = curl
```

### Pass

makefile:

```make
.POSIX:
PKG = curl

all:
	apt-get install -y $(PKG)
```

provision.include.mk:

```make
PKG = curl
```

### Mitigation

* Declare at least one non-special rule in most makefiles.
* Rename include files to `*.include.mk`.

## RULE_ALL

make interprets the first non-special rule as the default rule. Apart from non-special targets, the top-most rule is conventionally named `all`. This helps to avoid confusion and accidents.

### Fail

```make
.POSIX:

build:
	echo "Hello World!"
```

### Pass

```make
.POSIX:

all:
	echo "Hello World!"
```

Optionally, list subsequent rules as prerequisites for the `all` target.

```make
.POSIX:

all: build

build:
	echo "Hello World!"
```

### Mitigation

* Name the top-most, non-special, default rule `all`.

## STRICT_POSIX

> To receive exactly the behavior described in this section, the user shall ensure that a portable makefile shall:
>
> • Include the special target .POSIX
>
> • Omit any special target reserved for implementations (a leading period followed by uppercase letters) that has not been specified by this section
>
> The behavior of make is unspecified if either or both of these conditions are not met.

It is good form to begin most makefiles with a `.POSIX:` special target rule marker. This marker instructs make implementations to preserve processing semantics as defined in the POSIX standard without alteration. Omitting the marker may result in unknown behavior. So most makefiles benefit from more predictable behavior by leading with `.POSIX:`. You can declare this marker at the very first line of a makefile, or after some blank/comment lines.

However, makefiles named `*.include.mk`, designed for simple text inclusion into other makefiles, should omit the `.POSIX:` marker.

Also, make distributions commonly install a `sys.mk` include file that provides defines a foundational set of macros, include lines, and rules for make implementations. A `.POSIX:` marker may not be necessary for make distribution files. As well, files named like `GNUmakefile`, that are known to be implementation-specific, should not use this marker. But most any POSIX makefile *not* named to indicate its intention as an include file, should feature the `.POSIX:` marker.

### Fail

makefile:

```make
PKG = curl
```

### Pass

makefile:

```make
.POSIX:
PKG = curl
```

provision.include.mk:

```make
PKG = curl
```

GNUmakefile:

```make
PKG = curl
```

Special targets like `.POSIX` and `.PHONY` are important, but they may be elided from other passing examples in this document, for brevity.

### Mitigation

* Declare `.POSIX:` in most makefiles.
* Rename makefiles intended for inclusion to `*.include.mk`.
* Avoid declaring `.POSIX:` in makefiles for specific implementations like `GNUmakefile`.

# Undefined Behavior (UB)

Linter warnings concerning UB level portability issues tend to carry **higher** risk compared to other warnings. This is a consequence of the POSIX standard not specifying any particular error handling (or error detection) semantic for make implementations to follow.

In the case of UB, a makefile may trigger an error message during certain project builds, silently skip processing, corrupt files, segfault, fire missiles, and/or any number of undefined behaviors.

## UB_LATE_POSIX_MARKER

> If it appears as the first non-comment line in the makefile, make shall process the makefile as specified by this section; otherwise, the behavior of make is unspecified.

When the `.POSIX:` rule is used in a makefile, it must be the first thing in the makefile, apart from any blank or commented lines.

### Fail

```make
PKG = curl
.POSIX:
```

```make
.POSIX:
.POSIX:
```

### Pass

```make
.POSIX:
PKG = curl
```

```make
.POSIX:
```

```make
PKG = curl
```

### Mitigation

* Move `.POSIX` to the first non-blank, non-commented line in the makefile.
* Avoid mixing the `.POSIX` target with other targets in a single rule declaration.
* Avoid declaring `.POSIX:` in makefiles intended for use in include lines.
* Avoid declaring `.POSIX:` multiple times.

## UB_AMBIGUOUS_INCLUDE

> This standard does not specify precedence between macro definition and include directives. Thus, the behavior of:
>
> `include =foo.mk`
>
>is unspecified.

Ambiguous include/macro instructions do not have a clear meaning. The instruction may behave as `include` the path `=foo.mk`, or behave as defining a macro with the name `include` and the value `foo.mk`. Parsing destabilizes.

### Fail

```make
include =foo.mk
```

### Pass

```make
include=foo.mk
```

```make
include foo.mk
```

```make
INCLUDE = include
$(INCLUDE) =foo.mk
```

```make
PTH = =foo.mk
include $(PTH)
```

### Mitigation

* Avoid using equals (`=`) in path names.
* Avoid using lowercase `include` as a macro name.
* Consider removing whitespace between macro names and assignment operators.

## UB_MAKEFLAGS_ASSIGNMENT

> The result of setting MAKEFLAGS in the Makefile is unspecified.

The `MAKEFLAGS` macro is designed as read-only, set aside for make implementations to store command line flags.

POSIX compliant make implementations automatically preserve command line flags with `MAKEFLAGS`.

make implementations *implicitly* forward `MAKEFLAGS` to any child `$(MAKE)` invocations on behalf of the makefile user.

### Fail

```make
MAKEFLAGS = -j

all:
	$(MAKE) $(MAKEFLAGS) foo.mk
```

### Pass

```make
all:
	$(MAKE) foo.mk
```

### Mitigation

* Avoid assigning to the `MAKEFLAGS` macro.
* Move complex logic to a dedicated script.

## UB_SHELL_MACRO

> The value of the SHELL environment variable shall not be used as a macro and shall not be modified by defining the SHELL macro in a makefile or on the command line.

`SHELL` provides low level functionality to make implementation internals. Expanding or assigning this macro is discouraged.

make implementations that use `SHELL`, tend to set useful defaults. Overriding the defaults may produce non-portable, fragile makefiles.

Some implementations do not define `SHELL`. Assigning a value `SHELL` can create an misleading, non-portable impression of makefile behavior.

Due to `unmake` not evaluating macro expansions, expansion of the `SHELL` macro is not implemented as an automatic check.

Some ancient platforms may present `SHELL` with a `cmd[.exe]` interpreter. But even Windows Command Prompt, the Chocolatey GNU make interpreter tends to default to a POSIX compliant `sh` interpreter suitable for use with makefile commands.

### Fail

```make
SHELL = sh
```

```make
all:
	$(SHELL) script.sh
```

```make
all:
	${SHELL} script.sh
```

### Pass

```make
all:
	./script.sh
```

```make
	sh -c "echo $$SHELL"
```

### Mitigation

* Avoid assignments to the `SHELL` makefile macro.
* Treat the `SHELL` makefile macro as a private, internal make macro
* Note that a distinct `SHELL` environment variable may be available to commands, apart from the `SHELL` make macro.
* Move complex shell logic to a dedicated shell script.
