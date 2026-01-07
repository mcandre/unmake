# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker](https://www.docker.com/) 20.10.12+
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [rustup](https://rustup.rs/)
* [Rust](https://www.rust-lang.org/en-US/)
* [GNU](https://www.gnu.org/software/tar/)/[BSD](https://man.freebsd.org/cgi/man.cgi?tar(1))/[Windows](https://ss64.com/nt/tar.html) tar with gzip support
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

# INSTALL BINARIES FROM SOURCE

```console
$ make install
```

# UNINSTALL BINARIES

```console
$ make uninstall
```

# SECURITY AUDIT

```console
$ make audit
```

# LINT

```console
$ make lint
```

# TEST

```console
$ make test
```

# PORT

```console
$ make port
```

# PUBLISH

```console
$ make publish
```

# TEST DOCKER IMAGES

```console
$ make docker-test
```

# PUSH DOCKER IMAGES

```console
$ make docker-push
```

# CLEAN

```console
$ make clean
```
