.POSIX:
.SILENT:
.IGNORE: uninstall clean
.PHONY: \
	all \
	audit \
	build \
	doc \
	cargo-audit \
	cargo-check \
	clean \
	clean_cargo \
	clean_example \
	clean_ports \
	clippy \
	crit \
	install \
	lint \
	port \
	publish \
	rustfmt \
	test \
	uninstall \
	unmake

BANNER=tinyrick-0.0.15

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean_cargo \
	clean_example \
	clean_ports

clean_cargo:
	cargo clean

clean_ports:
	crit -c

clean_example:
	rm -f example/Cargo.lock
	rm -rf example/target

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
	rustfmt \
	unmake

port: crit
	sh -c "cd .crit/bin && tar czf $(BANNER).tgz $(BANNER)"

publish:
	cargo publish

rustfmt:
	cargo fmt

test: install
	sh -c "cd example && tinyrick -l"
	sh -c "cd example && tinyrick -v"
	sh -c "cd example && tinyrick -h"
	sh -c "cd example && tinyrick"
	sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build_debug build_release build doc install unit_test integration_test test banner uninstall clean_cargo clean"

uninstall:
	cargo uninstall tinyrick

unmake:
	unmake .
	unmake -n .
