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
$ cd fixtures

$ unmake valid/makefile

$ unmake invalid/for-loop.BSDmakefile
error at 1:16: expected one of " ", "$(", "${", ":", "\t", [^ (' ' | '\t' | ':' | ';' | '#' | '\r' | '\n')]
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

`make` is a natural candidate for working around limitations in provisioning scripts. For example, `go mod` / `cargo` do not track linters or other dev dependencies, and `sh` defaults to ignoring errors during provisioning. `make`'s default semantics prepare it well for provisioning and other activities. `make` can do many things! `unmake` helps it do them better.

`unmake` discourages vendor locking in makefile scripts. Numerous makefiles online assume a highly specific development environment. For example, assuming that (GNU) findutils, (GNU) sed, (GNU) awk, (non-PowerShell) curl are installed, with a GNU bash or zsh user interpreter, on a GNU/Linux operating system. So the typical makefile is likely to fail for (non-WSL) Windows users, or macOS users, or FreeBSD users, and so on. Ideally, our makefiles strive for portability, so that our projects can be enjoyed on a wider variety of computers. Portability, naturally restricts `makefile` contents to a portable subset, of what the various make implementations allow people to write. But a portability linter can curtail some of the worst offenders.

The dream is for every `makefile` to behave as a *polyglot* script, cabable of running well on most any computer platform with a `make` implementation installed.

## Example projects using unmake

* [buttery](https://github.com/mcandre/buttery), a GIF looper
* [crit](https://github.com/mcandre/crit), a Rust cross-compiler
* [factorio](https://github.com/mcandre/factorio), a Go cross-compiler
* [octane](https://github.com/mcandre/octane), a MIDI forwarder
* [slick](https://github.com/mcandre/slick), a POSIX sh syntax validator

# PARSING

`unmake` follows a stiff reading of the POSIX `make` standard:

https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html

Briefly, characters in `makefiles` that are explicitly rejected by the standard, may be treated as parse errors. Implementation-defined behavior, undefined behavior, and certain ill-advised syntax, may be treated as parse errors.

Common examples of `makefile` syntax that may trigger parse errors:

* Rule declarations with zero prerequisites, zero inline commands, and zero indented commands are out of spec.
* Vintage macOS CR (`\r`) and Windows CRLF (`\r\n`) line endings are out of spec. If you have a need to contribute to projects with makefiles from a Windows machine, configure your text editor to use LF (`\n`) line endings (and a final LF as well).
* Spaces at the beginning of rule command lines, are out of spec.
* Macro assignments with no identifier (`= 1`) are out of spec.
* Plain leftover macro identifiers with no assignment (`A`) are out of spec.
* Include paths with double-quotes (`""`) are out of spec.

Certain escaped line feed sequences may trigger parse errors. For example:

* Attempts to backslash escape a line following a comment (`#...\\\n...`), are out of spec.
* Include lines featuring escaped newlines (`\\\n`) are rejected as undefined behavior, per the POSIX spec.
* Escapes that end on an cliffhanger (`\\<EOF>`) are out of spec.
* Trailing whitespace in the middle of an escaped line feed (`\\ \n`) is out of spec.
* Escaped line feeds in general expressions (`$X\\\n`...) are out of spec.
* Escaped line feeds in the middle of macro definition names, target names, and/or prerequisite names, are out of spec.

Certain extensions beyond the POSIX `make` subset, such as GNU-isms, or BSD-isms, etc., may also trigger parse errors.

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

## POSIX.1-202x Issue 8, draft 3

https://www.opengroup.org/austin/

POSIX is not a single standard, but a series of versioned standards. unmake targets the upcoming 202x edition of POSIX, specifically Issue 8, draft 3; alias Open Group Issue 8, draft 3; alias IEEE P1003.1-202x/D3.

unmake assumes that make implementations already fully comply with the standard, supporting all syntax semantics, and the latest features required by the standard.

## Logic Errors

Neither `make`, nor linters, can perfectly read the mind of the `makefile` author. A rule that is syntactically valid, but accidentally neglects to declare a relevant prerequisite, can result in `make` misbehaving.

A rule with zero prerequisites, zero inline commands, and zero indented commands, may trigger a parse error. But a syntactically valid rule that happens to omit relevant prerequisite tasks or files (e.g. C/C++ source files), can run poorly in `make`, and may not be safeguarded by any linter messages.

Users are expected to have working knowledge of `make`, including the basic semantics of how `make` resolves task trees.

## Timestamps

`make` caches artifact files using timestamps. In fact, the common `touch` command (UNIX, Windows) provides a useful way to force `make` to rebuild downstream targets. However, the POSIX standard observes that `make` can experience problems when run on platforms that lack sufficient, sub-second timestamp precision.

Neither `make` nor `unmake` are not expected to resolve problems caused by builds running on environments that lack a realtime clock with sufficient timestamp precision, such as when running builds directly on microcontrollers or other embedded devices.

Fortunately, some workarounds are available for problems arising from timestamp precision:

* Run `make` on a machine with a realtime clock. For example, use a conventional workstation to cross-compile. Then copy the resulting artifacts onto the embedded device.
* Mark the task `.PHONY`, which disables more caching features.
* Use a build tool other than the POSIX `make` implementation family.

### makefiles as prerequisites

`make` is quite dim about updating artifacts as a result of changes to the `makefile`.

For example, changing compiler flags in a `makefile` and then proceeding to build the project with `make`, is likely to result in a bad cache state, where artifacts fail to regenerate with the new compiler flags.

One workaround involves declaring the makefile as a prerequisite for every target artifact, though that may be a tedious exercise.

The more common workaround, is to remain *aware* of this silly quirk of `make`, and habitually `make clean; make; make install`, etc. immediately upon editing the makefile. This achieves an accurate artifact cache state, and helps to validate your makefile change sooner, rather than risking latent problems.

In fact, the build configuration of a software project tends to be modified at a lower frequency than the primary project code. As long as you remember to reset your artifacts and rebuild them anew after each makefile tweak, then you can get along just fine without the `makefile` itself as an explicit rule prerequisite.

# RUNTIME REQUIREMENTS

(None)

# CONTRIBUTING

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

FreeBSD

# NOTABLE MAKE'S

## GNU make

[GNU make](https://www.gnu.org/software/make/) is a mature make implementation with high market share. In fact, many projects that use makefiles require, either explicitly or implicitly, specifically the GNU implementation of make. This is the default implementation for many (GNU/)Linux distributions. On most Linux distributions, GNU make is often referred to simply as "make." Sometimes, GNU make is referred to as "gmake," especially on BSD distributions.

## BSD make

[BSD make](https://man.freebsd.org/cgi/man.cgi?make(1)) is another mature make implementation. This is the default make implementation for many BSD distributions. On most BSD distributions, BSD make is often referred to simply as "make." Sometimes, BSD make is referred to as "bmake," especially on Linux and macOS distributions.

macOS is an interesting exception. macOS derives from BSD, and tends to omit many components, features, CLI flags, and other aspects of GNU standards. Yet, macOS / Xcode make defaults to GNU make, not BSD make.

Homebrew and other package managers may offer the package and/or binary "bsdmake," which is a more complete port of the related "bmake" package/binary.

## nmake

Microsoft `nmake` is used in the context of some Visual Studio / .NET projects, though nmake's syntax for include lines and macros appears to be break compatibility with POSIX make.

### amake ... zmake

Many `*make` command line tools exist, and vary the gamut. Some are POSIX compliant `make` implementations, able to serve as drop-in replacements for `.POSIX:` strict `makefile`s.

Other `*make` tools generate makefiles.

Still other `*make` tools have nothing to do with `make` or even build systems as a concept. Despite appearances, a project named like `*make` does not necessarily connote a POSIX make implementation. When in doubt, refer to that tool's specific documentation.

# SEE ALSO

* [BSD make](https://man.freebsd.org/cgi/man.cgi?make(1)), a popular make implementation with BSD extensions
* [checkmake](https://github.com/mrtazz/checkmake), an experimental makefile linter
* [cmake](https://cmake.org/), a build system for C/C++ projects
* [GNU autotools](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html), a build system for Linux C/C++ projects
* [GNU make](https://www.gnu.org/software/make/), a popular make implementation with GNU extensions
* [Gradle](https://gradle.org/), a build system for JVM projects
* [gyp](https://gyp.gsrc.io/), an obsolete C/C++ build system
* [invoke](https://pypi.org/project/invoke/), a task runner for Python projects
* [lake](https://luarocks.org/modules/steved/lake), a task runner for Lua projects
* [Mage](https://magefile.org/), a task runner for Go projects
* [Meson](https://mesonbuild.com/), an alternative C/C++ build system
* [ninja](https://ninja-build.org/), a fast build system without conditionals
* [nmake](https://learn.microsoft.com/en-us/cpp/build/reference/nmake-reference?view=msvc-170) is not POSIX compliant, but can run some rather limited POSIX makefiles, which avoid include lines and macros.
* [npm](https://www.npmjs.com/), [Grunt](https://gruntjs.com/), Node.js task runners
* [nobuild](https://github.com/tsoding/nobuild), a convention for ad-hoc C/C++ build systems
* the [POSIX make](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html) standard
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [ShellCheck](https://www.shellcheck.net/), a linter for POSIX sh family shell scripts
* [slick](https://github.com/mcandre/slick), a POSIX sh syntax validator
* [tinyrick](https://github.com/mcandre/tinyrick), a task runner for Rust projects
