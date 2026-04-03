.POSIX:
.SILENT:
.PHONY: all

all:
	go install golang.org/x/tools/cmd/goimports@latest
	go install golang.org/x/tools/go/analysis/passes/shadow/cmd/shadow@latest
	go install golang.org/x/vuln/cmd/govulncheck@latest
	go install tool
	go mod tidy
