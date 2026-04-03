package mx

import (
	"github.com/magefile/mage/sh"
)

// UnitTest executes the Go unit test suite.
func UnitTest(args ...string) error {
	var as []string
	as = append(as, "test")
	as = append(as, args...)
	return sh.RunV("go", as...)
}
