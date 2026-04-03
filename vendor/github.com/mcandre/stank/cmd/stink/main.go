// Package main implements a CLI application for identifying script metadata.
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"

	"github.com/mcandre/stank"
)

var flagPrettyPrint = flag.Bool("pp", false, "Prettyprint smell records")
var flagEOL = flag.Bool("eol", false, "Report presence/absence of final end of line sequence")
var flagCR = flag.Bool("cr", false, "Report presence/absence of any CR/CRLF's")
var flagHelp = flag.Bool("help", false, "Show usage information")
var flagVersion = flag.Bool("version", false, "Show version information")

// Stinker holds configuration for a stinky walk.
type Stinker struct {
	// EOLCheck enables final line terminator checks.
	EOLCheck bool

	// CRCheck enables carriage return checks.
	CRCheck bool

	// PrettyPrint expands formatting.
	PrettyPrint bool

	// sniffer analyzes files.
	sniffer stank.Sniffer
}

// NewStinker returns a Stinker.
func NewStinker() Stinker {
	var stinker Stinker
	stinker.sniffer = stank.NewSniffer()
	return stinker
}

// Walk sniffs a path,
// printing the smell of the script.
//
// If PrettyPrint is false, then the smell is minified.
func (o Stinker) Walk(pth string, _ os.FileInfo, _ error) error {
	smell, err2 := o.sniffer.Sniff(pth, stank.SniffConfig{EOLCheck: o.EOLCheck, CRCheck: o.CRCheck})

	if err2 != nil && err2 != io.EOF {
		log.Print(err2)
		return err2
	}

	if smell.Directory {
		return nil
	}

	var smellBytes []byte

	if o.PrettyPrint {
		smellBytes, _ = json.MarshalIndent(smell, "", "  ")
	} else {
		smellBytes, _ = json.Marshal(smell)
	}

	smellJSON := string(smellBytes)
	fmt.Println(smellJSON)
	return nil
}

func main() {
	flag.Parse()
	stinker := NewStinker()

	if *flagPrettyPrint {
		stinker.PrettyPrint = true
	}

	if *flagEOL {
		stinker.EOLCheck = true
	}

	if *flagCR {
		stinker.CRCheck = true
	}

	switch {
	case *flagVersion:
		fmt.Println(stank.Version)
		os.Exit(0)
	case *flagHelp:
		flag.PrintDefaults()
		os.Exit(0)
	}

	paths := flag.Args()

	var err error

	for _, pth := range paths {
		err = filepath.Walk(pth, stinker.Walk)

		if err != nil && err != io.EOF {
			log.Print(err)
		}
	}
}
