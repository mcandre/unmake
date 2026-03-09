# unmake: a makefile linter

[![CloudFlare R2 install media downloads](https://img.shields.io/badge/Cloudflare-F28220?style=for-the-badge&logo=Cloudflare&logoColor=white&style=flat)](#download) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/unmake?label=crate%20downloads)](https://crates.io/crates/unmake) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/unmake/total?logo=github)](https://github.com/mcandre/unmake/releases) [![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/unmake)](https://hub.docker.com/r/n4jm4/unmake) [![docs.rs](https://img.shields.io/docsrs/unmake)](https://docs.rs/unmake/latest/unmake/) [![Test](https://github.com/mcandre/unmake/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/unmake/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

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
warning: ./missing-posix.mk:1: STRICT_POSIX: lead makefiles with the ".POSIX:" compliance marker, or else rename include files like *.include.mk
...
```

See `unmake -h` for more options.

See [makefile](makefile) for a live example of a portable dev environment provisioning script for this Rust project.

# DOWNLOAD

<table>
  <thead>
    <tr>
      <th>OS</th>
      <th colspan=2>Package</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Alpine Linux</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/alpine-linux/unmake-0.0.26-r1.x86_64.apk">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/alpine-linux/unmake-0.0.26-r1.aarch64.apk">ARM</a></td>
    </tr>
    <tr>
      <td>Fedora</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/fedora/unmake-0.0.26-1.x86_64.rpm">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/fedora/unmake-0.0.26-1.aarch64.rpm">ARM</a></td>
    </tr>
    <tr>
      <td>FreeBSD</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/freebsd-amd64/unmake-0.0.26_1.pkg">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>macOS</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/macos/unmake-x86_64-0.0.26-1.pkg">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/macos/unmake-arm64-0.0.26-1.pkg">ARM</a></td>
    </tr>
    <tr>
      <td>NetBSD</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/netbsd-x86_64/unmake-0.0.26nb1.tgz">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>Ubuntu</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/ubuntu/unmake_0.0.26-1_amd64.deb">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/ubuntu/unmake_0.0.26-1_arm64.deb">ARM</a></td>
    </tr>
    <tr>
      <td>Windows</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/windows/unmake-0.0.26.1-x64.msi">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.26/windows/unmake-0.0.26.1-arm64.msi">ARM</a></td>
    </tr>
  </tbody>
</table>

# SYSTEM REQUIREMENTS

## Bitness

64

For more host platforms and installation methods, see our [install guide](INSTALL.md).

# NOTABLE FEATURES

`unmake` applies a stricter reading of POSIX syntax than `make -n`. Whereas `make -n` may skip inactive sections depending on control flow, `unmake` scans each line. For example, `make -n` may only check instructions specific to building the default task.

In fact, the two checks *complement* each other. `make -n` checks for dry-run runtime issues. `unmake` checks for syntactic portability issues.

## Directory recursion

`unmake` automatically recurses over directories.

When recursing over directories, `unmake` skips symlinks.

`unmake`'s linter rules skip many implementation-specific files such as `GNUmakefile`.

`unmake` skips many *machine-generated* makefiles. For example, makefiles produced by autotools; Perl; and cmake when using the Unix Makefile generator (both in-source builds and out-of-source builds).

`unmake` skips any third party makefiles housed in subdirectories like `.git`, `node_modules`, or `vendor`.

To investigate makefiles in more detail, see the `--debug` or `--inspect` command line options for `unmake`.

# PARSE ERRORS

`unmake` can identify low level makefile quirks, such as invalid syntax.

See [SYNTAX.md](SYNTAX.md) for more information.

# DRY RUN INTEGRITY CHECK

`-n` / `--dry-run` performs passthrough dry run validation with external make implementation tools, e.g. `bmake -nf`, `gmake -nf`, `make -nf`, etc.

**Per POSIX, rule commands prefixed with plus (`+`) may continue to execute in dry run mode.**

A few benefits of unmake dry run checks:

* Catch POSIX make *superset* parse errors (e.g. GNU and BSD)
* Catch semantic errors, such as missing target definitions
* Automatically skip common machine generated makefiles from cmake, Perl, etc.
* Less fuss than manual `find` \ `xargs` snippets
* Reduce log noise

The unmake dry run option aggressively assumes that most makefiles are buildable, top level project configurations, as opposed to make include files named like `include.mk`, `*.include.mk`, etc.

# LIST MAKEFILES

`-l` / `--list` emits paths of any matching makefiles that unmake finds in the given file paths.

This is useful for feeding large make projects into external linters. Unfortunately, many linters are poorly designed, lacking directory recursion and automatic file type identification. As a stopgap, unmake can perform these duties, exporting a subset of makefiles within a large project, through `xargs`, to an external linter.

Like dry run, the list option automatically skips common machine generated makefiles.

When piping unmake makefile lists through xargs, we recommend adding a `--print0` flag to unmake, and adding a `-0` flag to xargs. This informs both programs to transfer data in null delimited form, as a precaution against errors related to any spaces in file paths.

# WARNINGS

`unmake` can identify higher level portability recommendations for makefiles.

See [WARNINGS.md](WARNINGS.md) for more information.

# RESOURCES

Prior art, personal plugs, and tools for developing software.

* [BSD make](https://man.freebsd.org/cgi/man.cgi?make(1)), a popular make implementation with BSD extensions
* [cmake](https://cmake.org/), a make-adjacent build system with its own portability features
* [GNU make](https://www.gnu.org/software/make/), a popular make implementation with GNU extensions
* [Grunt](https://gruntjs.com/), Node.js task runners
* [invoke](https://pypi.org/project/invoke/), a task runner for Python projects
* [lake](https://luarocks.org/modules/steved/lake), a task runner for Lua projects
* [mcandre/linters](https://github.com/mcandre/linters), a wiki of common programming language linters and SAST tools
* [mage](https://magefile.org/), a task runner for Go projects
* [ninja](https://ninja-build.org/), a fast build system without conditionals
* [nmake](https://learn.microsoft.com/en-us/cpp/build/reference/nmake-reference?view=msvc-170), a make-adjacent build system for .NET projects
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [ShellCheck](https://www.shellcheck.net/), a linter for POSIX sh family shell scripts

🙃
