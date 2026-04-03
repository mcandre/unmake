package mx

import (
	"bufio"
	"bytes"
	"os"
	"os/exec"
)

// GoListSourceFilesTemplate provides a standard Go template for querying
// a project's Go source file paths.
const GoListSourceFilesTemplate = "{{$p := .}}{{range $f := .GoFiles}}{{$p.Dir}}/{{$f}}\n{{end}}"

// GoListTestFilesTemplate provides a standard Go template for querying
// a project's Go test file paths.
const GoListTestFilesTemplate = "{{$p := .}}{{range $f := .XTestGoFiles}}{{$p.Dir}}/{{$f}}\n{{end}}"

// GoPaths models information about Go project source trees.
type GoPaths struct {
	// All collects Go source file paths.
	All map[string]bool

	// Regular collects non-unit-test Go source file paths.
	Regular map[string]bool

	// Test collects unit test Go source file paths.
	Test map[string]bool
}

// NewGoPaths constructs a GoPaths.
func NewGoPaths() GoPaths {
	return GoPaths{
		All:     make(map[string]bool),
		Regular: make(map[string]bool),
		Test:    make(map[string]bool),
	}
}

// NoVendor queries Go source file paths,
// excluding vendored paths.
func NoVendor() (*GoPaths, error) {
	var sourceOut bytes.Buffer
	var testOut bytes.Buffer

	cmdSource := exec.Command(
		"go",
		"list",
		"-f",
		GoListSourceFilesTemplate,
		"./...",
	)
	cmdSource.Env = os.Environ()
	cmdSource.Stderr = os.Stderr
	cmdSource.Stdout = &sourceOut

	if err := cmdSource.Run(); err != nil {
		return nil, err
	}

	gopaths := NewGoPaths()
	scannerSource := bufio.NewScanner(&sourceOut)

	for scannerSource.Scan() {
		pth := scannerSource.Text()

		gopaths.All[pth] = true
		gopaths.Regular[pth] = true
	}

	cmdTest := exec.Command(
		"go",
		"list",
		"-f",
		GoListTestFilesTemplate,
		"./...",
	)
	cmdTest.Env = os.Environ()
	cmdTest.Stderr = os.Stderr
	cmdTest.Stdout = &testOut

	if err := cmdTest.Run(); err != nil {
		return nil, err
	}

	scannerTest := bufio.NewScanner(&testOut)

	for scannerTest.Scan() {
		pth := scannerTest.Text()

		gopaths.All[pth] = true
		gopaths.Test[pth] = true
	}

	return &gopaths, nil
}
