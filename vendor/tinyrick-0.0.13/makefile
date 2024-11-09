.POSIX:
.SILENT:
.IGNORE: uninstall clean
.PHONY: all \
	audit \
	doc \
	lint \
	clippy \
	rustfmt \
	unmake \
	build \
	port \
	crit \
	test \
	install \
	uninstall \
	publish \
	clean

BANNER=tinyrick-0.0.13

all: build

test: install
	sh -c "cd example && tinyrick -l"
	sh -c "cd example && tinyrick -v"
	sh -c "cd example && tinyrick -h"
	sh -c "cd example && tinyrick"
	sh -c "cd example && VERBOSE=1 tinyrick test clippy lint build_debug build_release build doc install unit_test integration_test test banner uninstall clean_cargo clean"

install:
	cargo install --force --path .

uninstall:
	cargo uninstall tinyrick

audit:
	cargo audit

doc:
	cargo doc

clippy:
	cargo clippy

rustfmt:
	cargo fmt

unmake:
	unmake .

lint: doc clippy rustfmt unmake

build: lint test
	cargo build --release

publish:
	cargo publish

crit:
	crit -b $(BANNER)

port: crit
	sh -c "cd .crit/bin && zip -r $(BANNER).zip $(BANNER)"

clean:
	crit -c
	cargo clean
	rm -rf example/target
	rm -rf example/Cargo.lock
