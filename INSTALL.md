# INSTALL

In addition to OS packages, unmake also supports alternative installation methods.

# INSTALL (CARGO)

unmake is packaged as a Rust crate.

```sh
cargo install unmake
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

# INSTALL (DOCKER)

unmake is packaged as a [Docker Hub](https://hub.docker.com/r/n4jm4/unmake) image.

```sh
docker pull n4jm4/unmake
```

## Prerequisites

* [Docker](https://www.docker.com/)

# INSTALL (CURL)

unmake supports curl based installs.

```sh
curl -L https://raw.githubusercontent.com/mcandre/unmake/refs/heads/main/install-unmake | sh
```

## Postinstall

Ensure `$HOME/.local/bin` is registered with your shell's `PATH` environment variable.

## Uninstall

```sh
curl -L https://raw.githubusercontent.com/mcandre/unmake/refs/heads/main/uninstall-unmake | sh
```

## System Requirements

### Bitness

64

### Operating Systems

* FreeBSD 13 (Intel)
* Illumos (Intel)
* Linux (ARM, Intel)
* macOS 26 Tahoe+ (ARM, Intel)
* NetBSD 10.1 (Intel)
* WSL 2 (ARM, Intel)

### Prerequisites

* [bash](https://www.gnu.org/software/bash/) 4+
* [curl](https://curl.se/)

# INSTALL (PRECOMPILED BINARIES)

Precompiled binaries may be installed manually.

## Install

1. Download a [tarball](https://github.com/mcandre/unmake/releases) corresponding to your environment's architecture and OS.
2. Extract executables into a selected directory.

   Examples:

   * `~/.local/bin` (XDG compliant per-user)
   * `/usr/local/bin` (XDG compliant global)
   * `~/bin` (BSD)
   * `~\AppData\Local` (native Windows)

## Postinstall

Ensure the selected directory is registered with your shell's `PATH` environment variable.

## Uninstall

Remove the application executables from the selected directory.

## System Requirements

### Bitness

64

### Operating Systems

* FreeBSD 13 (Intel)
* Illumos (Intel)
* Linux (ARM, Intel)
* macOS 26 Tahoe+ (ARM, Intel)
* NetBSD 10.1 (Intel)
* Windows 11+ (ARM, Intel)

# INSTALL (COMPILE FROM SOURCE)

unmake may be compiled from source.

```sh
git clone https://github.com/mcandre/unmake.git
cd unmake
cargo install --force --path .
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)
* [git](https://git-scm.com/)

For more details on developing unmake, see our [development guide](DEVELOPMENT.md).
