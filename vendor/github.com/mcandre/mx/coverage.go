package mx

import (
	"fmt"

	"github.com/magefile/mage/sh"
)

// CoverageHTML generates HTML formatted coverage data.
func CoverageHTML(htmlFilename string, profileFilename string) error {
	return sh.RunV(
		"go",
		"tool",
		"cover",
		fmt.Sprintf("-html=%s", profileFilename),
		"-o",
		htmlFilename,
	)
}

// CoverageProfile generates raw coverage data.
func CoverageProfile(profileFilename string) error {
	return sh.RunV(
		"go",
		"test",
		fmt.Sprintf("-coverprofile=%s", profileFilename),
	)
}
