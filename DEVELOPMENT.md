# DEVELOPMENT

We follow standard, `cargo` based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# PREREQUISITES

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [awscli](https://aws.amazon.com/cli/)
* [bash](https://www.gnu.org/software/bash/) 4+
* [Docker](https://www.docker.com/)
* [Go](https://go.dev/)
* [jq](https://jqlang.org/)
* [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [rustup](https://rustup.rs/)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [asdf](https://asdf-vm.com/)

## Postinstall

Register output of `go env GOBIN` to `PATH` environment variable.

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
make
```

## Install

```sh
make install
```

## Uninstall

```sh
make uninstall
```

## Security Audit

```sh
make audit
```

## Lint

```sh
make lint
```

## Test

```sh
make test
```

## Crosscompile Binaries

```sh
make crit
```

## Package Binaries

```sh
make package
```

## Upload Packages

```sh
make upload
```

## Publish Crate

```sh
make publish
```

## Clean Workspace

```sh
make clean
```
