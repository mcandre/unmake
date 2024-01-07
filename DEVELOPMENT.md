# BUILDTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 20.10.12+
* POSIX compatible [make](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html)
* [rustup](https://rustup.rs/) 1.25.2+
* [Rust](https://www.rust-lang.org/en-US/) 1.75.0+
* [zip](https://en.wikipedia.org/wiki/ZIP_(file_format))
* Provision additional dev tools with `make`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* [GNU time](https://www.gnu.org/software/time/)

# INSTALL BINARIES FROM SOURCE

```console
$ tinyrick install
```

# UNINSTALL BINARIES

```console
$ tinyrick uninstall
```

# SECURITY AUDIT

```console
$ tinyrick audit
```

# LINT

```console
$ tinyrick lint
```

# TEST

```console
$ tinyrick test
```

# PORT

```console
$ tinyrick port
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
