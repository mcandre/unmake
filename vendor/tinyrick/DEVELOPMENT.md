# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build.sh` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+
* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [coreutils](https://www.gnu.org/software/coreutils/coreutils.html)
* [zip](https://linux.die.net/man/1/zip)
* [Docker](https://www.docker.com/)

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ sh build.sh install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ sh build.sh uninstall
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ sh build.sh [build]
```

# PUBLISH

```console
$ sh build.sh publish
```

# PORT

```console
$ sh build.sh port
```

# CLEAN

```console
$ sh build.sh clean
```
