package mx

import (
	"github.com/magefile/mage/sh"
)

// GoEnv queries the `go env` toolchain configuration system.
func GoEnv(key string) (string, error) {
	return sh.Output("go", "env", key)
}
