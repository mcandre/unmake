# unmake: a makefile linter

```text
                   _
 _ _ ___ _____ ___| |_ ___
| | |   |     | .'| '_| -_|
|___|_|_|_|_|_|__,|_,_|___|
```

# ABOUT

`unmake` is a makefile linter emphasizing portability. We target the POSIX make standard.

With `unmake`, you can develop practical, portable `makefile` scripts, such as for provisioning dev environments.

No need for more heavyweight tools like Ansible, Docker, Lua, Python, or WSL! Just good ol' make.

# EXAMPLES

```console
$ cd fixtures/parse-valid

$ unmake .
warning: ./Makefile: MAKEFILE_PRECEDENCE: lowercase Makefile to makefile for launch speed
warning: ./boilerplate-ats.mk:4: SIMPLIFY_AT: replace individual at (@) signs with .SILENT target declaration(s)
warning: ./missing-posix.mk:1: STRICT_POSIX: lead makefiles with the ".POSIX:" compliance marker, or else rename include files to *.include.mk
...
```

See `unmake -h` for more options.

See [makefile](makefile) for a live example of a portable dev environment provisioning script for this Rust project.

# NOTABLE FEATURES

`unmake` applies a stricter reading of POSIX syntax than `make -n`. Whereas `make -n` may skip inactive sections depending on control flow, `unmake` scans each line. For example, `make -n` may only check instructions specific to building the default task.

In fact, the two checks *complement* each other. `make -n` checks for dry-run runtime issues. `unmake` checks for syntactic portability issues.

## Directory recursion

`unmake` automatically recurses over directories.

When recursing over directories, `unmake` skips symlinks.

`unmake` skips many implementation-specific files such as `GNUmakefile`.

`unmake` skips many *machine-generated* makefiles. For example, makefiles produced by autotools; Perl; and cmake when using the Unix Makefile generator (both in-source builds and out-of-source builds).

`unmake` skips any third party makefiles housed in subdirectories like `.git`, `node_modules`, or `vendor`.

To investigate makefiles in more detail, see the `--debug` or `--inspect` command line options for `unmake`.

# PARSE ERRORS

`unmake` can identify low level makefile quirks, such as invalid syntax.

See [SYNTAX.md](SYNTAX.md) for more information.

# WARNINGS

`unmake` can identify higher level portability recommendations for makefiles.

See [WARNINGS.md](WARNINGS.md) for more information.

# CRATE

https://crates.io/crates/unmake

# API DOCUMENTATION

https://docs.rs/unmake/latest/unmake/

# DOWNLOAD

https://github.com/mcandre/unmake/releases

# INSTALL FROM SOURCE

```console
$ cargo install --force --path .
```

# RUNTIME REQUIREMENTS

(None)

## Recommended

* [jq](https://stedolan.github.io/jq/)

# CONTRIBUTING

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

FreeBSD

# MORE EXAMPLES

Some projects using `unmake` to safeguard their makefiles:

* [buttery](https://github.com/mcandre/buttery), a GIF looper
* [crit](https://github.com/mcandre/crit), a Rust cross-compiler
* [factorio](https://github.com/mcandre/factorio), a Go cross-compiler
* [octane](https://github.com/mcandre/octane), a MIDI forwarder
* [slick](https://github.com/mcandre/slick), a POSIX sh syntax validator

# SEE ALSO

* [BSD make](https://man.freebsd.org/cgi/man.cgi?make(1)), a popular make implementation with BSD extensions
* [cmake](https://cmake.org/), a make-adjacent build system with its own portability features
* [GNU make](https://www.gnu.org/software/make/), a popular make implementation with GNU extensions
* [Grunt](https://gruntjs.com/), Node.js task runners
* [invoke](https://pypi.org/project/invoke/), a task runner for Python projects
* [lake](https://luarocks.org/modules/steved/lake), a task runner for Lua projects
* [mage](https://magefile.org/), a task runner for Go projects
* [ninja](https://ninja-build.org/), a fast build system without conditionals
* [nmake](https://learn.microsoft.com/en-us/cpp/build/reference/nmake-reference?view=msvc-170), a make-adjacent build system for .NET projects
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [remake](https://github.com/rocky/remake), a fork of GNU make that adds features like an interactive debugger
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [ShellCheck](https://www.shellcheck.net/), a linter for POSIX sh family shell scripts
* [slick](https://github.com/mcandre/slick), a POSIX sh syntax validator
* [tinyrick](https://github.com/mcandre/tinyrick), a task runner for Rust projects
