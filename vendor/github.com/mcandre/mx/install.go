package mx

import (
	"path"

	"github.com/magefile/mage/sh"
)

// Install builds and installs Go applications.
func Install(args ...string) error {
	var as []string
	as = append(as, "install")
	as = append(as, args...)
	as = append(as, "./...")
	return sh.RunV("go", as...)
}

// Uninstall deletes installed Go applications.
func Uninstall(applications ...string) error {
	goBin, err := GoEnv("GOBIN")

	if err != nil {
		return err
	}

	for _, application := range applications {
		if err := sh.Rm(path.Join(goBin, application)); err != nil {
			return err
		}
	}

	return nil
}
