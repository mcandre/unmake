# WARNINGS

`unmake` offers various checks to optimize your makefiles.

Note that `unmake` does not evaluate makefiles, and therefore ignores quirks arising from macro expansions.

# General

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

### Mitigation

* Declare `.POSIX:` in most makefiles.
* Rename makefiles intended for inclusion to `*.include.mk`.
* Avoid declaring `.POSIX:` in makefiles for specific implementations like `GNUmakefile`.

# Undefined Behavior (UB)

Linter warnings concerning UB level portability issues tend to carry higher risk than other warnings. This is a consequence of the POSIX standard not specifying any particular error handling (or error detection) semantic for make implementations to follow.

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
