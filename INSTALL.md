# INSTALL

We support several installation methods.

# PRECOMPILED BINARIES

https://github.com/mcandre/unmake/releases

## Requirements

(None)

## Instructions

1. Download release archive.
2. Extract archive.
3. Select executables for your target platform.
4. Copy executabless to a convenient location, e.g. `$HOME/bin`.
5. Ensure location is registered in `$PATH`.

# DOCKER

## Requirements

* [Docker](https://www.docker.com/) 28.0.1+

## Instructions

```sh
docker pull n4jm4/unmake
```

# BUILD FROM SOURCE

## Requirements

* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+

## Instructions

```sh
cargo install --force --path .
```

For more details on developing unmake itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
