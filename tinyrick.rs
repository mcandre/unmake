//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

use std::path;

/// archive bundles executables.
fn archive() {
    tinyrick_extras::archive(
        path::Path::new(".crit").join("bin").display().to_string(),
        banner(),
    );
}

/// Security audit
fn audit() {
    tinyrick_extras::cargo_audit();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Build: Doc, lint, test, and compile
fn build() {
    tinyrick::deps(lint);
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// Run cargo check
fn cargo_check() {
    tinyrick_extras::cargo_check();
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_cargo);
    tinyrick::deps(clean_ports);
}

/// Clean cargo
fn clean_cargo() {
    tinyrick_extras::clean_cargo();
}

/// Clean ports
fn clean_ports() {
    assert!(
        tinyrick::exec_mut!("crit", &["-c"])
            .status()
            .unwrap()
            .success()
    );
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(audit);
    tinyrick::deps(install);
    tinyrick::deps(cargo_check);
    tinyrick::deps(clippy);
    tinyrick::deps(doc);
    tinyrick::deps(rustfmt);
}

/// Lint, and then install artifacts
fn install() {
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Prepare cross-platform release media.
fn port() {
    tinyrick_extras::crit(vec!["-b".to_string(), banner()]);
    tinyrick::deps(archive);
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Run rustfmt
fn rustfmt() {
    tinyrick_extras::rustfmt();
}

/// Run tests
fn test() {
    tinyrick_extras::unit_test();
}

/// Uninstall artifacts
fn uninstall() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        archive,
        audit,
        cargo_check,
        clean,
        clean_cargo,
        clean_ports,
        clippy,
        doc,
        install,
        lint,
        port,
        publish,
        rustfmt,
        test,
        uninstall
    );
}
