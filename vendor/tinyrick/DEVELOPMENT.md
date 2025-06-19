# OVERVIEW

tinyrick's own compilation process is compatible with standard `cargo`. We wrap some common workflows with `build` tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 20.10.21+
* [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/) compatible [make](https://en.wikipedia.org/wiki/Make_(software))
* [Rust](https://www.rust-lang.org/en-US/) 1.87.0+
* [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/) compatible [tar](https://en.wikipedia.org/wiki/Tar_(computing))
* Provision additional dev tools with `make -f install.mk [-j 4]`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after provisioning)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [make](https://en.wikipedia.org/wiki/Make_(software))
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [tar](https://en.wikipedia.org/wiki/Tar_(computing))
* a [UNIX](https://en.wikipedia.org/wiki/Unix)-like environment

# INSTALL BINARY ARTIFACTS FROM LOCAL SOURCE

```console
$ make install
```

# UNINSTALL BINARY ARTIFACTS

```console
$ make uninstall
```

# SECURITY AUDIT

```console
$ make audit
```

# BUILD: LINT, DOC, COMPILE, and TEST

```console
$ make build
```

# PUBLISH

```console
$ make publish
```

# PORT

```console
$ make port
```

# CLEAN

```console
$ make clean
```
