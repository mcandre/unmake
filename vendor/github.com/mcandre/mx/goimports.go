package mx

import (
	"github.com/magefile/mage/sh"
)

// GoImports runs goimports.
func GoImports(args ...string) error {
	gopaths, err := NoVendor()

	if err != nil {
		return err
	}

	for pth := range gopaths.All {
		var as []string
		as = append(as, args...)
		as = append(as, pth)

		if err := sh.RunV("goimports", as...); err != nil {
			return err
		}
	}

	return nil
}
