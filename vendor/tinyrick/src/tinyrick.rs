//! CLI tinyrick tool

extern crate getopts;
extern crate tinyrick;

use std::env;
use std::path;
use std::process;

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

/// Show version information
pub fn banner() {
  println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// CLI entrypoint
fn main() {
  let arguments : Vec<String> = env::args()
    .collect();

  let brief = format!("Usage: {} [options]", env!("CARGO_PKG_NAME"));

  let mut opts : getopts::Options = getopts::Options::new();
  opts.optflag("l", "list", "list available tasks");
  opts.optflag("h", "help", "print usage info");
  opts.optflag("v", "version", "print version info");

  match opts.parse(&arguments[1..]) {
    Err(_) => {
      usage(&brief, &opts);
      process::abort();
    },
    Ok(optmatches) => {
      let list_tasks = optmatches.opt_present("l");

      if optmatches.opt_present("h") {
        usage(&brief, &opts);
        process::exit(0);
      } else if optmatches.opt_present("v") {
        banner();
        process::exit(0);
      } else {
          let tasks = optmatches.free;

          tinyrick::exec!(
            "cargo",
            &[
              "build",
              "--bin", env!("CARGO_PKG_NAME"),
              "--features", tinyrick::FEATURE
            ]
          );

        let target_path : &path::Path = path::Path::new("target");

        let rick_pathbuf : path::PathBuf = target_path
          .join("debug")
          .join(&format!("{}{}", env!("CARGO_PKG_NAME"), tinyrick::binary_suffix()));

        let rick_path : &str = rick_pathbuf
          .to_str()
          .unwrap();

        if list_tasks {
          tinyrick::exec!(rick_path, &["-l"]);
        } else {
          tinyrick::exec!(rick_path, tasks);
        }
      }
    }
  }
}
