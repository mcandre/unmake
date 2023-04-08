#!/bin/bash
set -euo pipefail

usage() {
    echo -e "Usage: $0 <arch> <libc>\n"

    echo -e "Architectures:\n"
    echo -e "x86_64\n"

    echo -e "libc's:\n"
    echo "gnu"
    echo "musl"

    exit 1
}

if [ "$#" -ne 2 ]; then
    usage
fi

arch="$1"
libc="$2"

host_root_src_dir="$(cd "$(dirname "$0")" && pwd)"

docker run -v "${host_root_src_dir}:/src" \
    "mcandre/docker-rustup:${arch}-${libc}" \
    sh -c "cd /src && cargo build --release --target ${arch}-unknown-linux-${libc}"
