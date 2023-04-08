//! Premade tasks for common tinyrick workflows

extern crate tinyrick;

/// Run clippy
pub fn clippy() {
    tinyrick::exec!("cargo", &["clippy"]);
}

/// Run rustfmt
pub fn rustfmt() {
    tinyrick::exec!("cargo", &["fmt"]);
}

/// Build debug binaries
pub fn build_debug() {
    tinyrick::exec!("cargo", &["build"]);
}

/// Build release binaries
pub fn build_release() {
    tinyrick::exec!("cargo", &["build", "--release"]);
}

/// Build all binaries
pub fn build() {
    tinyrick::deps(build_debug);
    tinyrick::deps(build_release);
}

/// Generate Rust API documentation
pub fn doc() {
    tinyrick::exec!("cargo", &["doc"]);
}

/// Install applications
pub fn install_binaries() {
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Generate cross-platform binaries.
pub fn crit(args: Vec<String>) {
    assert!(tinyrick::exec_mut!("crit", args)
        .status()
        .unwrap()
        .success());
}

/// Uninstall artifacts
pub fn uninstall_binaries() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run unit tests
pub fn unit_test() {
    tinyrick::exec!("cargo", &["test"]);
}

/// Publish to crate repository
pub fn publish() {
    tinyrick::exec!("cargo", &["publish"]);
}

/// Run cargo clean
pub fn clean_cargo() {
    tinyrick::exec!("cargo", &["clean"]);
}
