# tinyrick_extras: common tasks for tinyrick projects

# EXAMPLE

```console
$ cd example

$ tinyrick
running 1 test
test smoketest ... ok
...
Value(94)
Buzz
Fizz
Value(97)
Value(98)
Fizz
Buzz
```

# ABOUT

tinyrick_extras defines some common tasks, such as unit tests, linting, generating API documentation, publishing packages, installing and uninstalling packages, for your [tinyrick](https://github.com/mcandre/tinyrick) projects. Boom. Take what works for your build workflow, leave the rest.

Check out the [example](example) project.

# CRATE

https://crates.io/crates/tinyrick_extras

# API DOCUMENTATION

https://docs.rs/tinyrick_extras/latest/tinyrick_extras/

# LICENSE

BSD-2-Clause

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.87.0+
* [tinyrck](https://github.com/mcandre/tinyrick) 0.0.15

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [crit](https://github.com/mcandre/crit) ports Rust applications
* [direnv](https://direnv.net/) 2

# CONTRIBUTING

For more details on developing tinyrick_extras itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
