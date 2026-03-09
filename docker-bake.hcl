variable "VERSION" {
    default = "test"
}

variable "PLATFORMS" {
    # Drop 32-bit support
    # Work around buildx quirks
    default = [
        # "linux/386",
        "linux/amd64",
        # "linux/arm/v6",
        # "linux/arm/v7",
        "linux/arm64/v8",
        # "linux/ppc64le",
        # "linux/riscv64",
        # "linux/s390x",
    ]
}

variable "PRODUCTION" {
    default = [
        "unmake",
    ]
}

variable "TEST" {
    default = [
        "test-unmake",
    ]
}

group "production" {
    targets = PRODUCTION
}

group "test" {
    targets = TEST
}

group "all" {
    targets = concat(PRODUCTION, TEST)
}

target "unmake" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/unmake:${VERSION}",
        "n4jm4/unmake",
    ]
}

target "test-unmake" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/unmake:test",
    ]
}
