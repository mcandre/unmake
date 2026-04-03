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
	govulncheck \
	install \
	lint \
	package \
	publish \
	rustfmt \
	shellcheck \
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

audit: cargo-audit govulncheck

build:
	cargo build --release

cargo-audit:
	cargo audit

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

govulncheck:
	govulncheck -scan package ./...

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt \
	shellcheck

package:
	rockhopper -r "version=$(VERSION)"

publish:
	cargo publish

rustfmt:
	cargo fmt

shellcheck:
	stank -print0 . | \
		xargs -0 -n 1 shellcheck

test:
	cargo test

uninstall:
	cargo uninstall unmake

upload:
	./upload
