# DEVELOPMENT

We follow standard, `go` based operations for compiling and unit testing Go code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# PREREQUISITES

* [Go](https://go.dev/)
* [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [asdf](https://asdf-vm.com/)

## Postinstall

Register output of `go env GOBIN` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
mage
```

## Install

```sh
mage install
```

## Uninstall

```sh
mage uninstall
```

## Security Audit

```sh
mage audit
```

## Lint

```sh
mage lint
```

## Test

```sh
mage test
```
