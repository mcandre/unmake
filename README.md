# unmake: a makefile linter

```text
                   _
 _ _ ___ _____ ___| |_ ___
| | |   |     | .'| '_| -_|
|___|_|_|_|_|_|__,|_,_|___|
```

# WARNING

Work in progress

# ABOUT

`unmake` is a makefile linter that promotes extreme portability.

Too many makefiles online are restricted to building only as Works On My Machine^TM.

`unmake` bucks this trend, encouraging the makefile author to think more critically about what level of platform support they want for their software builds.

Do you want to be able to build your software on macOS? On Linux? On FreeBSD? On Windows (Command Prompt, PowerShell, *and/or* WSL)? On `fish` terminals?

All of the above?

`unmake` can help to catch vendor-lock issues earlier in the SDLC process. So that your apps can build more reliably, for more contributors to enjoy.

# EXAMPLES

```console
$ cd examples

$ unmake Makefile; echo "$?"
0

$ unmake bsd/makefile; echo "$?"
error at 1:16: expected one of " ", "$(", "${", ":", "\t", [^ (' ' | '\t' | ':' | ';' | '#' | '\r' | '\n')]
1
```

See `unmake -h` for more options.

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

# MOTIVATION

The `unmake` linter serves several purposes.

`unmake` provides a strict replacement for `make -n`, in case the local `make` implementation has BSD, GNU, etc. extensions. `unmake` encourages validating makefiles for syntactic wholeness as part of their projects' linter suites, before any tasks are actually run.

`unmake` encourages long or subtle shell snippets to be moved to dedicated shell script files, where they are more amenable to scanning with shell script linters. Most linters for common shell snippet wrapping languages (e.g., Ansible, Dockerfile, makefile, Vagrantfile, various CI/CD pipelines) perform very limited scanning for potential flaws in the embedded snippets, compared with linters that specifically scan shell script files.

`unmake` discourages vendor locking in makefile scripts. Numerous makefiles online assume a highly specific development environment. For example, assuming that (GNU) findutils, (GNU) sed, (GNU) awk, (non-PowerShell) curl are installed, with a GNU bash or zsh user interpreter, on a GNU/Linux operating system. So the typical makefile is likely to fail for (non-WSL) Windows users, or macOS users, or FreeBSD users, and so on. Ideally, our makefiles strive for portability, so that our projects can be enjoyed on a wider variety of computers.

`make` is a natural candidate for working around limitations in provisioning scripts. For example, `go mod` / `cargo` do not track linters or other dev dependencies, and `sh` defaults to ignoring errors during provisioning. `make`'s default semantics prepare it well for provisioning and other activities. `make` can do many things! `unmake` helps it do them better.

# PARSING

`unmake` follows a stiff reading of the POSIX `make` standard:

https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html

Briefly, characters in `makefiles` that are explicitly rejected by the standard, may be treated as parse errors. Implementation-defined behavior, undefined behavior, and certain ill-advised syntax, may be treated as parse errors.

Common examples of `makefile` syntax that

* Vintage macOS CR (`\r`) and Windows CRLF (`\r\n`) line endings are out of spec. If you have a need to contribute to projects with makefiles from a Windows machine, configure your text editor to use LF (`\n`) line endings (and a final LF as well).
* Spaces (` `) en lieu of hard tabs (`\t`) at the beginning of rule commands, are out of spec.
* Whitespace in the middle of a backslash escaped line feed sequence (`\\ \n`) is out of spec.
* makefiles that end on a cliffhanger backslash escaped line feed sequence with no accompanying followup line in the same file (`\\\n<eof>`), are out of spec.
* Macro assignments with no identifier (e.g., `=1`) are out of spec.
* Plain leftover macro identifiers with no assignment (e.g., `A`) are out of spec.
* Include paths with double-quotes (`"`) are out of spec.
* Backslash escaped line feed sequences in include lines (`include`...`\\\n`) are out of spec.

Certain extensions beyond the POSIX `make` subset, such as GNU-isms, or BSD-isms, etc., may also trigger parse errors.

Repeat: This is a linter focusing on extreme portability. We break things in testing, so that your software breaks less often in production.

# LINTER WARNINGS

Coming soon.

# CAVEATS

We do our best to catch POSIX make violations, but some may slip by. For example:

* POSIX violations hiding inside macro expansions
* POSIX violations hiding inside command output
* Violations hiding inside chains of `include` directives
* Misuse of reserved target names
* Behavior during live `make` script execution
* An ever-growing list of GNU/BSD/etc. extensions to POSIX make

# RUNTIME REQUIREMENTS

(None)

# CONTRIBUTING

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

FreeBSD

# SEE ALSO

* [BSD make](https://man.freebsd.org/cgi/man.cgi?make(1)), a popular make implementation with BSD extensions
* [checkmake](https://github.com/mrtazz/checkmake), an experimental makefile linter
* [cmake](https://cmake.org/), a build system for C/C++ projects
* [dale](https://github.com/mcandre/dale), a task runner for D projects
* [GNU autotools](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html), a build system for Linux C/C++ projects
* [GNU make](https://www.gnu.org/software/make/), a popular make implementation with GNU extensions
* [Gradle](https://gradle.org/), a build system for JVM projects
* [invoke](https://pypi.org/project/invoke/), a task runner for Python projects
* [lake](https://luarocks.org/modules/steved/lake), a task runner for Lua projects
* [Mage](https://magefile.org/), a task runner for Go projects
* [npm](https://www.npmjs.com/), [Grunt](https://gruntjs.com/), Node.js task runners
* the [POSIX make](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html) standard
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [rez](https://github.com/mcandre/rez), a task runner for C/C++ projects
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [tinyrick](https://github.com/mcandre/tinyrick), a task runner for Rust projects
* [vast](https://github.com/mcandre/vast), a task runner for sh projects
