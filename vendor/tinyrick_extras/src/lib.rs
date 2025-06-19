//! Premade tasks for common tinyrick workflows

extern crate tinyrick;

/// Compress binaries.
///
/// artifacts_path denotes a build directory root,
/// where a software project houses porting artifacts.
///
/// port_basename denotes an archive directory root within the artifacts_path,
/// generally of the form "<app-name>-<version>".
pub fn archive(artifacts_path: String, port_basename: String) {
    let artifacts_path_str: &str = &artifacts_path;
    let port_basename_str: &str = &port_basename;
    let archive_basename: &str = &format!("{}.tgz", port_basename_str);
    assert!(
        tinyrick::exec_mut!("tar", &["czf", archive_basename, port_basename_str])
            .current_dir(artifacts_path_str)
            .status()
            .unwrap()
            .success()
    );
}

/// Build all binaries
pub fn build() {
    tinyrick::deps(build_debug);
    tinyrick::deps(build_release);
}

/// Build debug binaries
pub fn build_debug() {
    tinyrick::exec!("cargo", &["build"]);
}

/// Build release binaries
pub fn build_release() {
    tinyrick::exec!("cargo", &["build", "--release"]);
}

/// Run cargo audit
pub fn cargo_audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Run cargo check
pub fn cargo_check() {
    tinyrick::exec!("cargo", &["check"]);
}

/// Run cargo clean
pub fn clean_cargo() {
    tinyrick::exec!("cargo", &["clean"]);
}

/// Run clippy
pub fn clippy() {
    tinyrick::exec!("cargo", &["clippy"]);
}

/// Generate cross-platform binaries.
pub fn crit(args: Vec<String>) {
    assert!(
        tinyrick::exec_mut!("crit", args)
            .status()
            .unwrap()
            .success()
    );
}

/// Generate Rust API documentation
pub fn doc() {
    tinyrick::exec!("cargo", &["doc"]);
}

/// Install applications
pub fn install_binaries() {
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Publish to crate repository
pub fn publish() {
    tinyrick::exec!("cargo", &["publish"]);
}

/// Run rustfmt
pub fn rustfmt() {
    tinyrick::exec!("cargo", &["fmt"]);
}

/// Uninstall artifacts
pub fn uninstall_binaries() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run unit tests
pub fn unit_test() {
    tinyrick::exec!("cargo", &["test"]);
}
