# DEVELOPMENT GUIDE

mx follows standard, Go based operations for compiling and unit testing Go code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* [Go](https://go.dev/)
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* Provision additional dev tools with `make`

## Recommended

* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* macOS [open](https://ss64.com/mac/open.html) or equivalent alias

# AUDIT

```sh
mage audit
```

# TEST

```sh
mage test
```

# COVERAGE

```sh
mage coverageHTML
```

# LINT

```sh
mage lint
```

# CLEAN

```sh
mage clean
```
