.POSIX:
.SILENT:
.PHONY: all

all:
	cargo install --force \
		cargo-audit \
		cargo-cache \
		cargo-edit \
		chandler@0.0.9 \
		crit@0.0.14 \
		tuggy@0.0.28
	cargo install --force \
		cross \
			--git https://github.com/cross-rs/cross \
			--rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
	rustup component add \
		clippy \
		rustfmt
