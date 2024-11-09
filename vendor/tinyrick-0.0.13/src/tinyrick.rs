//! CLI tinyrick tool

extern crate die;
extern crate getopts;
extern crate tinyrick;

use die::{die, Die};
use std::env;
use std::path;

/// Show version information
pub fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// CLI entrypoint
fn main() {
    let brief: String = format!("Usage: {} [options]", env!("CARGO_PKG_NAME"));

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag("l", "list", "list available tasks");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let list_tasks: bool = optmatches.opt_present("l");
    let tasks: Vec<String> = optmatches.free;

    tinyrick::exec!(
        "cargo",
        &[
            "build",
            "--bin",
            env!("CARGO_PKG_NAME"),
            "--features",
            tinyrick::FEATURE
        ]
    );

    let target_path: &path::Path = path::Path::new("target");

    let rick_pathbuf: path::PathBuf = target_path.join("debug").join(format!(
        "{}{}",
        env!("CARGO_PKG_NAME"),
        tinyrick::binary_suffix()
    ));

    let rick_path: &str = rick_pathbuf.to_str().unwrap();

    if list_tasks {
        tinyrick::exec!(rick_path, &["-l"]);
        die!(0);
    }

    tinyrick::exec!(rick_path, tasks);
}
