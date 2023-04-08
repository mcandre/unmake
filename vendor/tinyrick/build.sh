#!/bin/sh
#
# Avoid make in order to work around https://github.com/rust-lang/rust/issues/52801

PACKAGE='tinyrick'
VERSION='0.0.9'
ARCHIVE="${PACKAGE}-${VERSION}.zip"
BIN="target/debug/$PACKAGE"

test() {
		install &&
				sh -c "cd example && tinyrick -l" &&
				sh -c "cd example && tinyrick -v" &&
				sh -c "cd example && tinyrick -h" &&
				sh -c "cd example && tinyrick" &&
				sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build_debug build_release build doc install unit_test integration_test test banner uninstall clean_cargo clean"
}

install_binaries() {
		cargo install --force --path .
}

install() {
		install_binaries
}

uninstall() {
		cargo uninstall tinyrick
}

clippy() {
		cargo clippy
}

doc() {
		cargo doc
}

lint() {
		doc &&
				clippy
}

build() {
		lint &&
				test &&
				cargo build --release
}

publish() {
		cargo publish
}

crosscompile() {
		sh crosscompile-linux.sh x86_64 gnu &&
				sh crosscompile-linux.sh x86_64 musl
}

port() {
		crosscompile &&
				zip "$ARCHIVE" \
						target/x86_64-unknown-linux-gnu/release/tinyrick target/x86_64-unknown-linux-musl/release/tinyrick
}

clean_example() {
		rm -rf example/target;
				rm -rf example/Cargo.lock
}

clean_cargo() {
		cargo clean
}

clean_ports() {
		rm *.zip
}

clean() {
		clean_example;
				clean_cargo;
				clean_ports
}

if [ -z "$1" ]; then
		build
fi

for task in "$@"; do
		"$task"
done
