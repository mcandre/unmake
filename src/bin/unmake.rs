//! CLI unmake tool

extern crate die;
extern crate getopts;
extern crate unmake;

use self::unmake::ast;
use die::{die, Die};
use std::env;
use std::fs;
use std::path;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "Usage: {} <OPTIONS> <makefile> [<makefile> ...]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
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

    let pth_strings: Vec<String> = optmatches.free;

    if pth_strings.is_empty() {
        die!(1; usage);
    }

    let mut found_quirk = false;

    for pth_string in pth_strings {
        let pth: &path::Path = path::Path::new(&pth_string);
        let md: fs::Metadata = fs::metadata(pth).die("unable to access file path");

        if md.is_dir() {
            die!(1; usage);
        }

        let makefile_str: &str = &fs::read_to_string(pth).die("unable to read makefile");

        if let Err(err) = ast::parse_posix(&pth_string, makefile_str) {
            found_quirk = true;
            eprintln!("{}", err);
        };
    }

    if found_quirk {
        die!(1);
    }
}
