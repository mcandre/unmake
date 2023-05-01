# WARNINGS

`unmake` offers various checks to optimize your makefiles.

# General

## MAKEFILE_PRECEDENCE

> By default, the following files shall be tried in sequence: ./makefile and ./Makefile.

--POSIX 202x Issue 8/D3

Using the lowercase filename `makefile` loads slightly faster than the capitalized filename `Makefile`. The lowercase naming reduces strain on filesystem requests.

### Fail

Makefile:

```make
PKG = curl
```

### Pass

makefile:

```make
PKG = curl
```

### Mitigation

* Rename `Makefile` to `makefile`

## POSIX_MARKER

> To receive exactly the behavior described in this section, the user shall ensure that a portable makefile shall:
>
> • Include the special target .POSIX
>
> • Omit any special target reserved for implementations (a leading period followed by uppercase letters) that has not been specified by this section
>
> The behavior of make is unspecified if either or both of these conditions are not met.

It is good form to begin most makefiles with a `.POSIX:` special target rule marker. This marker instructs make implementations to preserve processing semantics as defined in the POSIX standard without alteration. Omitting the marker may result in unknown behavior. So most makefiles benefit from more predictable behavior by leading with `.POSIX:`. You can declare this marker at the very first line of a makefile, or after some blank/comment lines.

However, makefiles intended for simple text inclusion into other makefiles, may omit the `.POSIX:` marker. As well, files named like `GNUmakefile`, that are known to be implementation-specific, should not use this marker.

Due to the ambiguity of whether a makefile is intended for direct use vs inclusion, this policy is not implemented as an automatic check. Likewise, `unmake` does not scan for spurious `.POSIX:` declarations in extended syntax files like `GNUmakefile`, because of the performance loss and requirement to handle additional grammars.

### Fail

makefile:

```make
PKG = curl
```

### Pass

makefile:

```make
.POSIX:
PKG = curl
```

GNUmakefile:

```make
PKG = curl
```

### Mitigation

* Declare `.POSIX:` in most makefiles
* Avoid declaring `.POSIX:` in makefiles intended for inclusion
* Consider naming makefiles intended for inclusion as `*.include.mk`, `*.include.GNUmakefile`, etc.
* Avoid declaring `.POSIX:` in makefiles for specific implementations like `GNUmakefile`

# Undefined Behavior (UB)

Linter warnings concerning UB level portability issues tend to carry higher risk than other warnings. This is a consequence of the POSIX standard not specifying any particular error handling (or error detection) semantic for make implementations to follow.

In the case of UB, a makefile may trigger an error message during certain project builds, silently skip processing, corrupt files, segfault, fire missiles, and/or any number of undefined behaviors.

## UB_LATE_POSIX_MARKER

> If it appears as the first non-comment line in the makefile, make shall process the makefile as specified by this section; otherwise, the behavior of make is unspecified.

When the `.POSIX:` rule is used in a makefile, it must be the first thing in the makefile, apart from any blank or commented lines.

### Fail

```make
PKG = curl
.POSIX:
```

```make
.POSIX:
.POSIX:
```

### Pass

```make
.POSIX:
PKG = curl
```

```make
.POSIX:
```

```make
PKG = curl
```

### Mitigation

* Move `.POSIX` to the first non-blank, non-commented line in the makefile.
* Avoid mixing the `.POSIX` target with other targets in a single rule declaration.
* Avoid declaring `.POSIX:` in makefiles intended for use in include lines.
* Avoid declaring `.POSIX:` multiple times.
