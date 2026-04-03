// Package main implements a CLI application to recursively identify POSIX shell scripts
// in large, complex directories.
//
// The user may then feed these file paths into shell script linters.
// (Many linters fail to implement recursion or basic file extension / shebang language detection.)
package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/mcandre/stank"
)

var flagSh = flag.Bool("sh", false, "Limit results to specifically bare POSIX sh scripts")
var flagAlt = flag.Bool("alt", false, "Limit results to specifically alternative, non-POSIX lowlevel shell scripts")
var flagExcludeInterpreters = flag.String("exInterp", "", "Remove results with the given interpreter(s) (Comma separated)")
var flagPrint0 = flag.Bool("print0", false, "Delimit file path results with a null terminator for conjunction with xargs -0")
var flagHelp = flag.Bool("help", false, "Show usage information")
var flagVersion = flag.Bool("version", false, "Show version information")

// StankMode controls stank rule behavior.
type StankMode int

const (
	// ModePOSIXy matches POSIX-like shell scripts.
	ModePOSIXy StankMode = iota

	// ModePureSh matches specifically sh-interpreted scripts.
	ModePureSh

	// ModeAltShellScript matches certain non-POSIX shell scripts.
	ModeAltShellScript
)

// Stanker holds configuration for a stanky walk
type Stanker struct {
	// Mode is scan type.
	Mode StankMode

	// InterpreterExclusions remove results from scan report.
	InterpreterExclusions []string

	// Printer writes file path results.
	Printer func(string)

	// sniffer analyzes files.
	sniffer stank.Sniffer
}

// NewStanker constructs a Stanker.
func NewStanker() Stanker {
	var stanker Stanker
	stanker.Mode = ModePOSIXy
	stanker.sniffer = stank.NewSniffer()
	return stanker
}

// LineWriter emits file paths with line terminators.
func LineWriter(pth string) {
	fmt.Println(pth)
}

// NullWriter emits file paths with explicit null terminators.
func NullWriter(pth string) {
	os.Stdout.Write([]byte(pth))
	os.Stdout.Write([]byte{0x00})
}

// Walk sniffs a file system node for POSIXyness.
// If the file smells sufficiently POSIXy, the path is printed.
// Otherwise, the path is omitted.
func (o Stanker) Walk(pth string, _ os.FileInfo, _ error) error {
	smell, err2 := o.sniffer.Sniff(pth, stank.SniffConfig{})

	if err2 != nil && err2 != io.EOF {
		log.Print(err2)
	}

	if stank.Ignore(pth) {
		return nil
	}

	if smell.MachineGenerated {
		return nil
	}

	for _, interpreterExclusion := range o.InterpreterExclusions {
		if smell.Interpreter == interpreterExclusion {
			return nil
		}
	}

	switch o.Mode {
	case ModePureSh:
		if smell.POSIXy && (smell.Interpreter == "sh" || smell.Interpreter == "generic-sh") {
			o.Printer(smell.Path)
		}
	case ModeAltShellScript:
		if smell.AltShellScript {
			o.Printer(smell.Path)
		}
	default:
		if smell.POSIXy {
			o.Printer(smell.Path)
		}
	}

	return nil
}

func main() {
	flag.Parse()
	stanker := NewStanker()

	if *flagSh {
		stanker.Mode = ModePureSh
	}

	if *flagAlt {
		stanker.Mode = ModeAltShellScript
	}

	if *flagPrint0 {
		stanker.Printer = NullWriter
	} else {
		stanker.Printer = LineWriter
	}

	stanker.InterpreterExclusions = strings.Split(*flagExcludeInterpreters, ",")

	switch {
	case *flagVersion:
		fmt.Println(stank.Version)
		os.Exit(0)
	case *flagHelp:
		flag.PrintDefaults()
		os.Exit(0)
	}

	paths := flag.Args()

	var observedError bool
	var err error

	for _, pth := range paths {
		err = filepath.Walk(pth, stanker.Walk)

		if err != nil {
			log.Print(err)
			observedError = true
		}
	}

	if observedError {
		os.Exit(1)
	}
}
