# tinyrick: a freeform Rust build system

```
       .---.              ^
     o{__ω__ o{          ^0^  -Let me out!
~~ ( // *|* \xx\)      xx`|'
        = =  xxxx&x      ' `
```

# EXAMPLE

```console
$ cd example

$ tinyrick
running 1 test
test smoketest ... ok

$ tinyrick -h
Usage: tinyrick [options]

Options:
    -l, --list          list available tasks
    -h, --help          print usage info
    -v, --version       print version info
```

# ABOUT

I'm tinyrick (TINYRICK!) and I build Rust projects. With tinyrick, you configure your build in the same normal Rust code as the rest of your project. Or keep picking your nose with make, it's up to you.

Look at my pants! tinyrick! You think my pants are one size fits all? No, of course not! So get the pants that fit you. Get a `tinyrick.rs` that fits your workflow. Task dependency trees, get em while they're hot! Segfaults, get em while they're not. Smarter, butter, faster, stranger.

Don't shell out, lib out. Your build is more portable that way. tinyricktinyricktinyrick. If you look closely, that last period is actually a *micro* rick rendered in ASCII; even tinier tinyrick!

# CRATE

https://crates.io/crates/tinyrick

# API DOCUMENTATION

https://docs.rs/tinyrick/latest/tinyrick/

# LICENSE

BSD-2-Clause

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.87.0+

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)

# SETUP

## tinyrick.rs

Write some tasks in a `tinyrick.rs` build configuration script at the top-level directory of your Rust project:

```rust
fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn test() {
    tinyrick::exec!("cargo", &["test"]);
}

fn build() {
    tinyrick::deps(test);
    tinyrick::exec!("cargo", &["build", "--release"]);
}

fn publish() {
    tinyrick::exec!("cargo", &["publish"]);
}

fn clean() {
    tinyrick::exec!("cargo", &["clean"]);
}

fn main() {
    tinyrick::phony!(clean);
    tinyrick::wubba_lubba_dub_dub!(build; banner, test, publish, clean);
}
```

## Cargo.toml

Now, wire up the tinyrick command line interface by configuring your top-level `Cargo.toml`:

```toml
[package]
name = "derpmobile"
description = "hyperadvanced derpmobiles"
version = "3.1.4"

[dependencies]
tinyrick = { version = "0.0.15", optional = true }

[features]
letmeout = ["tinyrick"]

[[bin]]
name = "tinyrick"
path = "tinyrick.rs"
required-features = ["letmeout"]
```

Launch a terminal session in your project directory. Install and run the tinyrick tool:

```console
$ cargo install tinyrick
$ tinyrick
```

Watch how he behaves... I hope tinyrick is practicing good manners :P

What happens when you run:

* `tinyrick banner`?
* `tinyrick test`?
* `tinyrick clean`?
* `tinyrick build`?
* `tinyrick -h`?
* `tinyrick --list`?
* `VERBOSE=1 tinyrick build`?

I bet the freakin' moon explodes if you run `VERBOSE=1 tinyrick build build build`! (Hold onto your pants~)

# DEBRIEFING

Where are my pants? Let's break down the code so far:

* `fn name() { ... }` declares a task named `name`.
* `deps(requirement)` caches a dependency on task `requirement`.
* `exec!(...)` spawns raw shell command processes.
* `VERBOSE=1` enables command string emission during processing.
* `phony!(...)` disables dependency caching for some tasks.
* `wubba_lubba_dub_dub!(default; ...)` exposes a `default` task and some other tasks to the tinyrick command line.
* `letmeout` is a feature gate, so that neither the tinyrick package, nor your tinyrick binary escape with your Rust package when you `tinyrick publish`.

# DoN't UsE sHelL cOmMaNdS!1

Just because the tinyrick library offers several *supremely convenient* macros for executing shell commands doesn't mean that you should always shell out. No way, man!

Whenever possible, use regular Rust code, as in the `banner()` example. There's like a ba-jillion [crates](https://crates.io) of prewritten Rust code, so you might as well use it!

# CONTRIBUTING

For more details on developing tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# SEE ALSO

* Inspired by the excellent [mage](https://magefile.org/) build system for Go projects
* [bb](https://github.com/mcandre/bb), a build system for (g)awk projects
* [beltaloada](https://github.com/mcandre/beltaloada), a guide to writing build systems for (POSIX) sh
* [booty](https://github.com/mcandre/booty?tab=readme-ov-file) for JS/Node.js/altJS
* [cargo](https://doc.rust-lang.org/cargo/reference/build-scripts.html) custom build scripts, primarily for generating Rust source files from other languages
* [cmake](https://cmake.org/) for C/C++ projects
* [dale](https://github.com/mcandre/dale) builds D projects
* [GNU autotools](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html), a build system for Linux C/C++ projects
* [Gradle](https://gradle.org/), a build system for JVM projects
* [invoke](https://pypi.org/project/invoke/), a Python task runner
* [jelly](https://github.com/mcandre/jelly), a JSON task runner
* [lake](https://luarocks.org/modules/steved/lake), a Lua task runner
* [Leiningen](https://leiningen.org/) + [lein-exec](https://github.com/kumarshantanu/lein-exec), a Clojure task runner
* [lichen](https://github.com/mcandre/lichen), a sed task runner
* [POSIX make](https://pubs.opengroup.org/onlinepubs/009695299/utilities/make.html), a task runner standard for C/C++ and various other software projects
* [mian](https://github.com/mcandre/mian), a task runner for (Chicken) Scheme Lisp
* [Rake](https://ruby.github.io/rake/), a task runner for Ruby projects
* [Rebar3](https://www.rebar3.org/), a build system for Erlang projects
* [rez](https://github.com/mcandre/rez) builds C/C++ projects
* [sbt](https://www.scala-sbt.org/index.html), a build system for Scala projects
* [Shake](https://shakebuild.com/), a task runner for Haskell projects
* [yao](https://github.com/mcandre/yao), a task runner for Common LISP projects

# EVEN MORE EXAMPLES

* The included [example](example) project provides a fully qualified demonstration of how to build projects with tinyrick.
* For a more practical example, see [ios7crypt-rs](https://github.com/mcandre/ios7crypt-rs), a little *modulino* library + command line tool for *deliciously dumb* password encryption.
* [tinyrick_extras](https://github.com/mcandre/tinyrick_extras) defines some common workflow tasks as plain old Rust functions, that you can sprinkle onto your tinyrick just like any other Rust crate.
