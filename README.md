# unmake: POSIX makefile linter

[![CloudFlare R2 install media downloads](https://img.shields.io/badge/Packages-F38020?logo=Cloudflare&logoColor=white)](#download) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/unmake?label=crate%20downloads)](https://crates.io/crates/unmake) [![docs.rs](https://img.shields.io/docsrs/unmake)](https://docs.rs/unmake/latest/unmake/) [![Test](https://github.com/mcandre/unmake/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/unmake/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

```text
                   _
 _ _ ___ _____ ___| |_ ___
| | |   |     | .'| '_| -_|
|___|_|_|_|_|_|__,|_,_|___|
```

# SUMMARY

Recursive POSIX makefile linter

# EXAMPLES

```console
% cd fixtures/parse-valid

% unmake .
warning: ./Makefile: MAKEFILE_PRECEDENCE: lowercase Makefile to makefile for launch speed
warning: ./boilerplate-ats.mk:4: SIMPLIFY_AT: replace individual at (@) signs with .SILENT target declaration(s)
warning: ./missing-posix.mk:1: STRICT_POSIX: lead makefiles with the ".POSIX:" compliance marker, or else rename include files like *.include.mk
...
```

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
      <td>macOS 26 Tahoe+</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.27/macos/unmake-x86_64-0.0.27-1.pkg">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.27/macos/unmake-arm64-0.0.27-1.pkg">ARM</a></td>
    </tr>
    <tr>
      <td>Ubuntu 24.04 Noble+ / WSL 2+</td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.27/ubuntu/unmake_0.0.27-1_amd64.deb">Intel</a></td>
      <td><a href="https://pub-a96ed3474b8e4de8ae61496c32ab08b6.r2.dev/unmake-0.0.27/ubuntu/unmake_0.0.27-1_arm64.deb">ARM</a></td>
    </tr>
  </tbody>
</table>

For more platforms and installation methods, see [INSTALL](INSTALL.md).

For details on parser rules, see [SYNTAX](SYNTAX.md).

For details on scanner rules, see [WARNINGS](WARNINGS.md).

For details on building from source, see [DEVELOPMENT](DEVELOPMENT.md).

# ABOUT

unmake detects subtle portability quirks in makefiles.

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
