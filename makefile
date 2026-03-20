.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clippy \
	crit \
	doc \
	install \
	lint \
	package \
	publish \
	rustfmt \
	test \
	uninstall \
	upload
.IGNORE: \
	clean \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages

VERSION!=cargo metadata --format-version 1 --no-deps | jq -r ".packages[0].version"
BANNER=unmake

all: build

audit:
	cargo audit

build:
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-cargo \
	clean-crit \
	clean-example

clean-cargo:
	cargo clean

clean-crit:
	crit -c

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	rm -rf example/.crit

clean-packages:
	rockhopper -c

clippy:
	cargo clippy

crit:
	crit -b $(BANNER)

doc:
	cargo doc

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

package:
	rockhopper -r "version=$(VERSION)"

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

uninstall:
	cargo uninstall unmake

upload:
	./upload
