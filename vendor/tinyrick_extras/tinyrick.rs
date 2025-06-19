//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

use std::fs;
use std::path;

/// Security audit
fn audit() {
    tinyrick_extras::cargo_audit();
}

/// Build: Doc, lint, test, and compile
fn build() {
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
    tinyrick::deps(clean_example);
}

/// Clean cargo
fn clean_cargo() {
    tinyrick_extras::clean_cargo();
}

/// Clean example
fn clean_example() {
    let pth_cargo_lock = path::Path::new("example").join("Cargo.lock");
    let pth_target = path::Path::new("example").join("target");
    let _ = fs::remove_file(pth_cargo_lock);
    let _ = fs::remove_dir_all(pth_target);
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
    tinyrick::deps(cargo_check);
    tinyrick::deps(clippy);
    tinyrick::deps(doc);
    tinyrick::deps(rustfmt);
    tinyrick::deps(unmake);
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Doc, lint, and run tests
fn test() {
    tinyrick::deps(lint);
    tinyrick_extras::unit_test();

    assert!(
        tinyrick::exec_mut!("tinyrick", &["build", "uninstall"])
            .current_dir("example")
            .env("VERBOSE", "1")
            .status()
            .unwrap()
            .success()
    );
}

/// Run rustfmt
fn rustfmt() {
    tinyrick_extras::rustfmt();
}

/// Run unmake
fn unmake() {
    tinyrick::exec!("unmake", &["."]);
    tinyrick::exec!("unmake", &["-n", "."]);
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        audit,
        cargo_check,
        clean,
        clean_cargo,
        clean_example,
        clippy,
        doc,
        lint,
        publish,
        rustfmt,
        test,
        unmake
    );
}
