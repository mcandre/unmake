package mx

import (
	"github.com/magefile/mage/sh"
)

// Nakedret runs nakedret.
func Nakedret(args ...string) error {
	var as []string
	as = append(as, args...)
	as = append(as, "./...")
	return sh.RunV("nakedret", as...)
}
