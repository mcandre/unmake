//go:build mage

package main

import (
	"github.com/magefile/mage/mg"
	"github.com/magefile/mage/sh"
	"github.com/mcandre/mx"
)

// Default references the default build task.
var Default = Build

// Audit runs a security audit.
func Audit() error { return Govulncheck() }

// Build compiles Go projects.
func Build() error { return sh.RunV("go", "build", "./...") }

// Deadcode runs deadcode.
func Deadcode() error { return sh.RunV("deadcode", "./...") }

// Errcheck runs errcheck.
func Errcheck() error { return sh.RunV("errcheck", "-blank") }

// GoImports runs goimports.
func GoImports() error { return mx.GoImports("-w") }

// GoVet runs default go vet analyzers.
func GoVet() error { return mx.GoVet() }

// Govulncheck runs govulncheck.
func Govulncheck() error { return sh.RunV("govulncheck", "-scan", "package", "./...") }

// Install builds and installs Go applications.
func Install() error { return mx.Install() }

// Lint runs the lint suite.
func Lint() error {
	mg.Deps(Deadcode)
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

// Test runs a unit test.
func Test() error { return mx.UnitTest() }

// Uninstall deletes installed Go applications.
func Uninstall() error { return mx.Uninstall("stink", "stank", "funk") }
