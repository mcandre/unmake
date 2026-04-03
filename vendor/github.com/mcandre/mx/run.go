package mx

import (
	"bytes"
	"io"
	"log"
	"os"

	"github.com/magefile/mage/sh"
)

// ExecSilent executes commands with the UNIX / Go idiom of silencing output unless of an error.
func ExecSilent(env map[string]string, program string, args ...string) error {
	var stdout bytes.Buffer
	var stderr bytes.Buffer
	_, err := sh.Exec(env, &stdout, &stderr, program, args...)

	if err != nil {
		if _, err2 := io.Copy(os.Stdout, &stdout); err2 != nil {
			log.Println(err2)
		}

		if _, err2 := io.Copy(os.Stderr, &stderr); err2 != nil {
			log.Println(err2)
		}
	}

	return err
}

// RunVSilent executes commands with the UNIX / Go idiom of silencing output unless of an error.
func RunVSilent(program string, args ...string) error {
	return ExecSilent(nil, program, args...)
}
