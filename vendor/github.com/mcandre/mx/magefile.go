//go:build mage

package main

import (
	"github.com/magefile/mage/mg"
	"github.com/magefile/mage/sh"
	"github.com/mcandre/mx"
)

// CoverHTML denotes the HTML formatted coverage filename.
const CoverHTML = "cover.html"

// CoverProfile denotes the raw coverage data filename.
const CoverProfile = "cover.out"

// Default references the default build task.
var Default = Test

// Audit runs a security audit.
func Audit() error { return Govulncheck() }

// Clean deletes build artifacts.
func Clean() error { mg.Deps(CleanCoverage); return nil }

// CleanCoverage deletes coverage data.
func CleanCoverage() error {
	if err := sh.Rm(CoverHTML); err != nil {
		return err
	}

	return sh.Rm(CoverProfile)
}

// CoverageHTML generates HTML formatted coverage data.
func CoverageHTML() error {
	mg.Deps(CoverageProfile)
	return mx.CoverageHTML(CoverHTML, CoverProfile)
}

// CoverageProfile generates raw coverage data.
func CoverageProfile() error { return mx.CoverageProfile(CoverProfile) }

// Errcheck runs errcheck.
func Errcheck() error { return sh.RunV("errcheck", "-blank") }

// Govulncheck runs govulncheck.
func Govulncheck() error { return sh.RunV("govulncheck", "-scan", "package", "./...") }

// GoImports runs goimports.
func GoImports() error { return mx.GoImports("-w") }

// GoVet runs default go vet analyzers.
func GoVet() error { return mx.GoVet() }

// Lint runs the lint suite.
func Lint() error {
	mg.Deps(GoImports)
	mg.Deps(GoVet)
	mg.Deps(Errcheck)
	mg.Deps(Nakedret)
	mg.Deps(Shadow)
	mg.Deps(Staticcheck)
	return nil
}

// Nakedret runs nakedret.
func Nakedret() error { return mx.Nakedret("-l", "0") }

// Shadow runs go vet with shadow checks enabled.
func Shadow() error { return mx.GoVetShadow() }

// Staticcheck runs staticcheck.
func Staticcheck() error { return sh.RunV("staticcheck", "./...") }

// Test executes the unit test suite.
func Test() error { return mx.UnitTest() }
