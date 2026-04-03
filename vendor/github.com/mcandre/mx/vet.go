package mx

import (
	"fmt"
	"os/exec"

	"github.com/magefile/mage/sh"
)

// GoVetShadow runs go vet against all Go packages in a project,
// with variable shadow checking enabled.
//
// Depends on golang.org/x/tools/go/analysis/passes/shadow/cmd/shadow
func GoVetShadow() error {
	shadowPath, err := exec.LookPath("shadow")

	if err != nil {
		return err
	}

	return GoVet(fmt.Sprintf("-vettool=%s", shadowPath))
}

// GoVet runs go vet against all Go packages in a project.
func GoVet(args ...string) error {
	var as []string
	as = append(as, "vet")
	as = append(as, args...)
	as = append(as, "./...")
	return sh.RunV("go", as...)
}
