# unmake: a makefile linter

```text
                   _
 _ _ ___ _____ ___| |_ ___
| | |   |     | .'| '_| -_|
|___|_|_|_|_|_|__,|_,_|___|
```

# ABOUT

`unmake` is a POSIX makefile linter emphasizing portability. `unmake` helps your software projects build more reliably on different machines.

# EXAMPLES

```console
$ cd fixtures

$ unmake valid/makefile

$ unmake invalid/crlf.mk
error: invalid/crlf.mk:1:5 found "\r", expected one of: ".WAIT", LF, comment, inline command, macro expansion, target

$ unmake -i valid/makefile | jq .
{
  "path": "/Users/andrew/go/src/github.com/mcandre/unmake/fixtures/valid/Makefile",
  "is_makefile": true,
  "build_system": "make",
  "is_machine_generated": false
}
```

See `unmake -h` for more options.

# PARSE ERRORS

`unmake` catches many subtle makefile quirks. If your makefile fails to parse with `unmake`, pay close attention to line endings, indentation, and any implementation-specific syntax.

See [SYNTAX.md](SYNTAX.md) for more information.

# LINTER WARNINGS

Coming soon.

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

# ABOUT

The `unmake` linter serves several purposes.

`unmake` provides a strict replacement for `make -n`, in case the local `make` implementation has BSD, GNU, etc. extensions. `unmake` encourages validating makefiles for syntactic wholeness as part of their projects' linter suites, before any tasks are actually run.

`unmake` encourages long or subtle shell snippets to be moved to dedicated shell script files, where they are more amenable to scanning with shell script linters. Most linters for common shell snippet wrapping languages (e.g., Ansible, Dockerfile, makefile, Vagrantfile, various CI/CD pipelines) perform very limited scanning for potential flaws in the embedded snippets, compared with linters that specifically scan shell script files.

`make` is a natural candidate for working around limitations in provisioning scripts. For example, `go mod` / `cargo` do not track linters or other dev dependencies, and `sh` defaults to ignoring errors during provisioning. `make`'s default semantics prepare it well for provisioning and other activities. `make` can do many things! `unmake` helps it do them better.

`unmake` discourages vendor locking in makefile scripts. Numerous makefiles online assume a highly specific development environment. For example, assuming that (GNU) findutils, (GNU) sed, (GNU) awk, (non-PowerShell) curl are installed, with a GNU bash or zsh user interpreter, on a GNU/Linux operating system. So the typical makefile is likely to fail for (non-WSL) Windows users, or macOS users, or FreeBSD users, and so on. Ideally, our makefiles strive for portability, so that our projects can be enjoyed on a wider variety of computers. Portability, naturally restricts `makefile` contents to a portable subset, of what the various make implementations allow people to write. But a portability linter can curtail some of the worst offenders.

The dream is for every `makefile` to behave as a *polyglot* script, cabable of running well on most any computer platform with a `make` implementation installed.

Note that certain build systems like Microsoft nmake, may not always follow POSIX syntax or semantics, even if their configuration files happened to be named `makefile`.

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
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [ShellCheck](https://www.shellcheck.net/), a linter for POSIX sh family shell scripts
* [slick](https://github.com/mcandre/slick), a POSIX sh syntax validator
* [tinyrick](https://github.com/mcandre/tinyrick), a task runner for Rust projects
