# die

[![Build Status](https://travis-ci.com/moparisthebest/die.svg?branch=master)](https://travis-ci.com/moparisthebest/die)
[![Latest Version](https://img.shields.io/crates/v/die.svg)](https://crates.io/crates/die)
[![Documentation](https://docs.rs/die/badge.svg)](https://docs.rs/die)

[die] is a simple Rust library to make it easy to handle errors and exit in command line programs.

[die]: https://code.moparisthebest.com/moparisthebest/die

```toml
# Cargo.toml
[dependencies]
die = "0.2"
```

Example usage:

```rust
use die::Die;
// Result:
Ok(1).die("no number"); // unwraps to 1 successfully
Err("failure").die("strange error"); // prints `strange error` to stderr then exits with code 1

// Option: 
Some(1).die("no number"); // unwraps to 1 successfully
None.die("none option"); // prints `none option` to stderr then exits with code 1

// custom error codes:
Err("failure").die_code("strange error", 4); // prints `strange error` to stderr then exits with code 4
None.die_code("none option", 5); // prints `none option` to stderr then exits with code 5

// die! macro:
die!("argument to -e must be numeric"); // prints message to stderr then exits with code 1
die!(2; "argument to -e must be numeric"); // prints message to stderr then exits with code 2
die!("argument to -e must be numeric"; 3); // prints message to stderr then exits with code 3
die!("argument {} must be {}", "-e", 1; 4); // prints `argument -e must be 1` to stderr then exits with code 4
die!("argument {} must be {}", "-e", 1); // prints `argument -e must be 1` to stderr then exits with code 1
die!(2); // prints nothing, only exits with code 3
die!(); // prints nothing, only exits with code 1
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in die by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
