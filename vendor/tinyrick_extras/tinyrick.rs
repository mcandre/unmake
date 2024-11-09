//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Security audit
fn audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
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

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
    tinyrick::deps(rustfmt);
    tinyrick::deps(unmake);
}

/// Doc, lint, and run tests
fn test() {
    tinyrick::deps(lint);
    tinyrick_extras::unit_test();

    assert!(tinyrick::exec_mut!("tinyrick", &["build", "uninstall"])
        .current_dir("example")
        .env("VERBOSE", "1")
        .status()
        .unwrap()
        .success());
}

/// Build: Doc, lint, test, and compile
fn build() {
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Clean workspaces
fn clean() {
    tinyrick_extras::clean_cargo();
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
      build;
      doc,
      audit,
      clippy,
      rustfmt,
      unmake,
      lint,
      test,
      publish,
      clean
    );
}
