.POSIX:
.PHONY: all

all:
	@rustup component add \
		clippy \
		rustfmt
	@cargo install --force \
		cargo-audit@0.17.5 \
		crit@0.0.6 \
		cross@0.2.5 \
		tinyrick@0.0.11
