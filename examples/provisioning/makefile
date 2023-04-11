all:
	@rustup component add clippy
	@rustup component add rustfmt
	@cargo install --force cargo-audit@0.17.5
	@cargo install --force tinyrick@0.0.11

	@go install github.com/alexkohler/nakedret@v1.0.1
	@go install github.com/kisielk/errcheck@v1.6.3
	@go install github.com/magefile/mage@v1.14.0
	@go install github.com/mcandre/factorio/cmd/factorio@v0.0.3
	@go install golang.org/x/lint/golint@latest
	@go install golang.org/x/tools/cmd/goimports@latest
	@go install golang.org/x/tools/go/analysis/passes/shadow/cmd/shadow@latest
	@go install honnef.co/go/tools/cmd/staticcheck@2023.1.3
	@go mod tidy

	@npm install -g snyk@1.1140.0
