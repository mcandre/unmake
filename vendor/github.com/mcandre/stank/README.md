# stank: shell script linters

[![go.dev reference](https://img.shields.io/badge/go.dev-reference-007d9c?logo=go&logoColor=white)](https://pkg.go.dev/github.com/mcandre/stank) [![Test](https://github.com/mcandre/stank/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/stank/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

# SUMMARY

stank recursively lints shell scripts.

# EXAMPLES

## Identify Shell Scripts

```console
% cd examples

% stank .
.profile
.shrc
.zlogin
...
```

## Recurse External Linters

```console
% stank -print0 . | xargs -0 -n 1 shellcheck
In welcome.sh line 1:
#!bash
^----^ SC2239 (error): Ensure the shebang uses an absolute path to the interpreter.

For more information:
  https://www.shellcheck.net/wiki/SC2239 -- Ensure the shebang uses an absolu...
```

For details on tuning stank, run `stank -help`.

# DOWNLOAD

```sh
go install github.com/mcandre/stank/cmd/...@latest
```

## Prerequisites

* [Go](https://go.dev/)

## Postinstall

Register output of `go env GOBIN` to `PATH` environment variable.

For details on building from source, see [DEVELOPMENT](DEVELOPMENT.md).

# ABOUT

stank is a library and collection of command line utilities for sniffing files to identify shell scripts like bash, sh, zsh, ksh and so on, those funky farmfresh gobs of garbaggio; versus other more palatable files like rb, py, pl.

Believe it or not, shell scripts are notoriously difficult to write well, so it behooves a developer to either write shell scripts in safer languages, or else wargame your scripts with an armada of linters. Trouble is, in large projects one can never be too sure which files are honest to dog POSIX compliant shell scripts, and which are pretenders. csh, tcsh, fish, ion, rc, and most other nonderivatives of bash tend to be NOT POSIX compatible. If you're geeky enough to have followed thus far, let's get crackalackin with some fruity examples dammit!

# MORE EXAMPLES

The `funk` linter reports strange odors emanating from scripts, such as improper line endings, the presence of Byte Order Marker's in some Unicode scripts.

```console
% funk examples
Ambiguous launch style. Either feature a file extensions, or else feature executable bits: examples/.shrc
Tokenize like `unset IFS` at the top of executable scripts: examples/.shrc
Control program flow like `set -euf` at the top of executable scripts: examples/.shrc
Tokenize like `unset IFS` at the top of executable scripts: examples/badconfigs/zprofile
Control program flow like `set -euf` at the top of executable scripts: examples/badconfigs/zprofile
Missing shebang: examples/blank.bash
Traps may reset in subshells: examples/cleanup.sh
Missing shebang: examples/goodbye.sh
Missing shebang: examples/greetings.bash
Control program flow like `set -euf` at the top of executable scripts: examples/hello-commented

% funk -modulino examples
Configuration features shebang: examples/badconfigs/.bash_profile
Configuration features executable permissions: examples/badconfigs/zprofile
Missing final end of line sequence: examples/blank.bash
Missing shebang: examples/blank.bash
Interpreter mismatch between shebang and extension: examples/derp.zsh
Missing shebang: examples/greetings.bash
Missing final end of line sequence: examples/hello-crlf.sh
CR/CRLF line ending detected: examples/hello-crlf.sh
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/hello-crlf.sh
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/howdy
Missing shebang: examples/howdy.zsh
Missing shebang: examples/just-eol.bash
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/lo
Missing final end of line sequence: examples/lo-cr.csh
CR/CRLF line ending detected: examples/lo-cr.csh
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/pipefail
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/shout.sh
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/wednesday
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/wednesday-bom
Leading BOM reduces portability: examples/wednesday-bom
Modulino ambiguity. Either have owner executable permissions with no extension, or else remove executable bits and use an extension like .lib.sh: examples/welcome
```

For details on tuning funk, run `funk -help`.

Both `stank` and `funk` have the ability to select low level, nonPOSIX scripts as well, such as csh/tcsh scripts used in FreeBSD.

Note that funk cannot reliably warn for missing shebangs if the extension is also missing; typically, script authors use one or the other to mark files as shell scripts. Lacking both a shebang and a file extension, means that a file could contain code for many languages, making it difficult to determine the POSIXy nature of the code. Even if an exhaustive set of ASTs are applied to test the file contents for syntactical validity across the dozens of available shell languages, there is a strong possibility in shorter files that the contents are merely incidentally valid script syntax, though the intent of the file is not to operate as a POSIX shell script. Short, nonPOSIX scripts such as for csh/tcsh could easily trigger a "POSIX" syntax match. In any case, know that the shebang is requisite for ensuring your scripts are properly interpreted.

Note that funk may fail to present permissions warnings if the scripts are housed on non-UNIX file systems such as NTFS, where executable bits are often missing from the file metadata altogether. When storing shell scripts, be sure to set the appropriate file permissions, and transfer files as a bundle in a tarball or similar to safeguard against dropped permissions.

Note that funk may warn of interpreter mismatches for scripts with extraneous dots in the filename. Rather than `.envrc.sample`, name the file `sample.envrc`. Rather than `wget-google.com`, name the file `wget-google-com`. Appending `.sh` is also an option, so `update.es.cluster` renames to `update.es.cluster.sh`.

The optional `-modulino` flag to funk enables strict separation of script duties, into distinct application scripts vs. library scripts. Application scripts are generally executed by invoking the path, such as `./hello` or `~/bin/hello` or simply `hello` when `$PATH` is appropriately modified. Application scripts feature owner executable permissions, and perhaps group and other as well depending on system configuration needs. In contrast, library scripts are intended to be imported with dot (`.`) or `source` into user shells or other scripts, and should feature a file extension like `.lib.sh`, `.sh`, `.bash`, etc. By using separate naming conventions, we more quickly communicate to downstream users how to interact with a shell script. In particular, by dropping file extensions for shell script applications, we encourage authors to choose more meaningful script names. Instead of the generic `build.sh`, choose `build-docker`. Instead of `kafka.sh`, choose `start-kafka`, `kafka-entrypoint`, etc.

Finally, `stink` prints a record of each file's POSIXyness, including any interesting fields it identified along the way. Note that some fields may be zero valued if the stench of POSIX or rosy waft of nonPOSIX is overwhelming, short-circuiting analysis. This short-circuiting feature dramatically speeds up how `stank` searches large projects.

Note that permissions are relayed as decimals, due to constraints on JSON integer formatting (we didn't want to use a custom octal string field). Use `echo 'obase=8;<some integer> | bc` to display these values in octal.

Note that legacy systems, packages, and shell scripts referencing "sh" may refer to a plethora of pre-POSIX shells. Modern systems rename "sh" to "lksh", "tsh", "etsh", etc. to avoid confusion. In general, the stank suite will assume that the majority of scripts being scanned are targeting post-1971 technology, so use your human intuition and context to note any legacy Thompson UNIX v6 "sh", etc. scripts. Most modern linters will neither be able to parse such scripts of any complexity, nor will they recognize them for the legacy scripts that they are, unless the scripts' shebangs are rendered with the modern retro interpreters "lksh", "tsh", "etsh", etc. for deployment on modern UNIX systems. One could almost use the fs stats for modification/change to try to identify these legacy outliers, but this is a practically unrealistic assumption except for the most obsessive archaeologist, diligently ensuring their legacy scripts continue to present 1970's metadata even after experimental content modifications. So the stank system will simply punt and assume sh -> POSIX sh, ksh -> ksh88 / ksh93 for the sake of modernity and balance.

Similarly, the old Bourne shell AKA "sh" AKA "bsh" presents language identification difficulties. Old Bourne shell scripts are most likely to present themselves with "sh" shebangs, which is okay as Bourne sh and ksh88/pdksh/ksh served as the bases for the POSIX sh standard. Some modern systems may present a Bourne shell as a "sh" or "bsh" binary. The former presents few problems for stank identification, though "bsh" is tricky, as the majority of its uses today are not associated with the Bourne shell but with the Java BeanShell. So stank may default to treating `bsh` scripts as non-POSIXy, and any such Bourne shell scripts are advised to feature either `bash` or `sh` shebangs, and perhaps `.sh` or `.bash` extensions, in order to self-identify as modern, POSIX compliant scripts.

```console
% stink examples/hello
{"Path":"examples/hello","Filename":"hello","Basename":"hello","Extension":"","Shebang":"#!/bin/sh","Interpreter":"sh","LineEnding":"\n","FinalEOL":false,"ContainsCR":false
,"Permissions":509,"Directory":false,"OwnerExecutable":true,"BOM":false,"POSIXy":true,"AltShellScript":false}

% stink -pp examples/hello
{
  "Path": "examples/hello",
  "Filename": "hello",
  "Basename": "hello",
  "Extension": "",
  "Shebang": "#!/bin/sh",
  "Interpreter": "sh",
  "LineEnding": "\n",
  "FinalEOL": false,
  "ContainsCR": false,
  "Permissions": 509,
  "Directory": false,
  "OwnerExecutable": true,
  "BOM": false,
  "POSIXy": true,
  "AltShellScript": false
}

% stink -pp examples/hello.py
{
  "Path": "examples/hello.py",
  "Filename": "hello.py",
  "Basename": "hello.py",
  "Extension": ".py",
  "Shebang": "#!/usr/bin/env python",
  "Interpreter": "python",
  "LineEnding": "\n",
  "FinalEOL": false,
  "ContainsCR": false,
  "Permissions": 420,
  "Directory": false,
  "OwnerExecutable": false,
  "BOM": false,
  "POSIXy": false,
  "AltShellScript": false
}
```

For details on tuning stink, run `stink -help`.

The included `examples/` directory demonstrates many edge cases, such as empty scripts, shebang-less scripts, extensioned and extensionless scripts, and various Hello World applications in across many programming languages. Some files, such as `examples/goodbye` may contain 100% valid POSIX shell script content, but fail to self-identify with either shebangs or relevant file extensions. In a large project, such files may be mistakenly treated as whoknowswhat format, or simply plain text. Perhaps statistical methods could help identify POSIX grammars, but even an empty file is technically POSIX, which is unhelpful from a reliable classification standpoint. In any case, `examples/` hopefully covers the more common edge cases.

One way to think of `stank` is a bounty hunter for shell scripts.

Given that shell tends to be more fragile than higher level programming languages, then it is a good idea to rewrite shell code as dedicated applications. Go and Rust are especially good choices for application languages.

The Rust programming language has best in class performance, reliability, and security. The Go programming language has comparable performance, reliability, and security in most contexts. Both Rust and Go support cross-compilation and static executables, so that it's much easier to develop, test, package, and distribute Rust/Go applications compared to flaky shell scripts. Most shell coders neglect to consider subtle vendor locking problems with shell syntax and the flags used for individual commands. Rust has a steeper learning curve than some coders are willing to devote time for. Often, Go can serve as a compromise. Being compiled languages, both Rust and Go are protected from many runtime pitfalls that shells and other interpreted languages invite.

Regardless, the particular programming language is a less important, concern, as long as it is not shell. Notoriously hazardous programming languages like JavaScript and Perl, are still safer than shell. Shell (any flavor) is a trash fire waiting for a spark.

Fortunately, the list of shell scripts that `stank` emits, can help engineers to identify program candidates to rewrite in more mature programming languages.

# WARNING ON FALSE NEGATIVES

Note that very many software components have a bad habit of encouraging embedded, inline shell script snippets into non-shell script files. For example, CI/CD job configurations, Dockerfile RUN steps, Kubernetes resources, and make. Most linter tools (for shell scripts and other languages) have very limited or nonexistent support for linting inline shell script snippets.

Accordingly, move shell script snippets to a dedicated shell script file. And then have the software component execute the shell script. Then you will be able to lint the shell code with more tools, and thereby raise the quality level of your system.

# WARNING ON FALSE POSITIVES

Some rather obscure files, such as Common Lisp source code with multiline, polyglot shebangs and no file extension, may falsely trigger the stank library, and the stink and stank applications, which short-circuit on the first line of the hacky shebang. Such files may be falsely identified as "POSIX" code, which is actually the intended behavior! This is because the polyglot shebang is a hack to work around limitations in the Common Lisp language, which ordinarily does not accept POSIX shebang comments, in order to get Common Lisp scripts to be dot-slashable in bash. For this situation, it is best to supply a proper file extension to such files.

```console
% head examples/i-should-have-an-extension
#!/usr/bin/env sh
#|
exec clisp -q -q $0 $0 ${1+"$@"}
|#

(defun hello-main (args)
  (format t "Hello from main!~%"))

;;; With help from Francois-Rene Rideau
;;; http://tinyurl.com/cli-args

% stink -pp examples/i-should-have-an-extension
{
  "Path": "examples/i-should-have-an-extension",
  "Filename": "i-should-have-an-extension",
  "Basename": "i-should-have-an-extension",
  "Extension": "",
  "BOM": false,
  "Shebang": "#!/usr/bin/env sh",
  "Interpreter": "sh",
  "LineEnding": "\n",
  "POSIXy": true
}
```

Perhaps append a `.lisp` extension to such files. Or separate the modulino into clear library vs. command line modules. Or extract the shell interaction into a dedicated script. Or convince the language maintainers to treat shebangs as comments. Write your congressman. However you resolve this, know that the current situation is far outside the norm, and likely to break in a suitably arcane and dramatic fashion. With wyverns and flaming seas and portents of all ill manner.

# RESOURCES

Prior art, personal plugs, and tools for developing applications (including non-shell projects)!

* [jq](https://jqlang.org/) - JSON transformer
* [mcandre/linters](https://github.com/mcandre/linters) - curated linter collection
