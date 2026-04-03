module github.com/mcandre/unmake

go 1.26.1

tool (
	github.com/mcandre/stank
	github.com/mcandre/stank/cmd/funk
	github.com/mcandre/stank/cmd/stank
	github.com/mcandre/stank/cmd/stink
)

require (
	github.com/magefile/mage v1.16.1 // indirect
	github.com/mcandre/mx v0.0.47 // indirect
	github.com/mcandre/stank v0.0.44 // indirect
	mvdan.cc/sh/v3 v3.13.0 // indirect
)
