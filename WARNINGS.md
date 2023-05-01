# WARNINGS

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
