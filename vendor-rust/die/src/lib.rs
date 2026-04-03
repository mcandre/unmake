// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process;

/// const of 1
pub const DEFAULT_EXIT_CODE: i32 = 1;

/// Prints a message to [`stderr`] and terminates the current process with the specified exit code
/// or 1 if no exit code is specified, by calling [`eprintln`]!() on all arguments followed by
/// [process::exit(exit_code)][exit]
///
/// [`eprintln`]: https://doc.rust-lang.org/std/macro.eprintln.html
/// [`stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
/// [exit]: https://doc.rust-lang.org/std/process/fn.exit.html
///
/// # Examples
///
/// Basic usage:
///
/// ```{.should_panic}
/// # use die::die;
/// die!("argument to -e must be numeric"); // prints message to stderr then exits with code 1
/// ```
/// With custom error code:
/// ```{.should_panic}
/// # use die::die;
/// die!(2; "argument to -e must be numeric"); // prints message to stderr then exits with code 2
/// ```
/// error code can go at the beginning or end, just separate with colon:
/// ```{.should_panic}
/// # use die::die;
/// die!("argument to -e must be numeric"; 3); // prints message to stderr then exits with code 3
/// ```
/// supports all the formatting eprintln! does:
/// ```{.should_panic}
/// # use die::die;
/// die!("argument {} must be {}", "-e", 1; 4); // prints `argument -e must be 1` to stderr then exits with code 4
/// ```
/// supports all the formatting eprintln! does without exit code too:
/// ```{.should_panic}
/// # use die::die;
/// die!("argument {} must be {}", "-e", 1); // prints `argument -e must be 1` to stderr then exits with code 1
/// ```
/// just exit with a code alone:
/// ```{.should_panic}
/// # use die::die;
/// die!(2); // prints nothing, only exits with code 3
/// ```
/// just exit:
/// ```{.should_panic}
/// # use die::die;
/// die!(); // prints nothing, only exits with code 1
/// ```
#[macro_export]
macro_rules! die {
    () => (::std::process::exit(::die::DEFAULT_EXIT_CODE));
    ($x:expr) => (::die::PrintExit::print_exit(&$x));
    ($x:expr; $y:expr) => (::die::PrintExit::print_exit(&($x, $y)));
    ($x:expr; $($y:expr),+) => ({
        eprintln!($($y),+);
        ::std::process::exit($x)
    });
    ($($y:expr),+; $x:expr) => ({
        eprintln!($($y),+);
        ::std::process::exit($x)
    });
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        ::std::process::exit(::die::DEFAULT_EXIT_CODE)
    });
}

/// `Die` is a trait implemented on [`Result`] and [`Option`] to make exiting with messages and codes easy
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub trait Die<T> {
    /// Unwraps a [`Result`] or [`Option`], yielding the content of an [`Ok`] or [`Some`].
    ///
    /// # Exits
    ///
    /// Calls [process::exit(1)][exit] if the value is an [`Err`] or [`None`], after printing the
    /// passed message to [`stderr`].
    ///
    /// [`stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
    /// [exit]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```{.should_panic}
    /// # use die::Die;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.die("strange error"); // prints `strange error` to stderr then exits with code 1
    /// ```
    /// ```{.should_panic}
    /// # use die::Die;
    /// let x: Option<u32> = None;
    /// x.die("strange error"); // prints `strange error` to stderr then exits with code 1
    /// ```
    fn die(self, msg: &str) -> T;

    /// Unwraps a [`Result`] or [`Option`], yielding the content of an [`Ok`] or [`Some`].
    ///
    /// # Exits
    ///
    /// Calls [process::exit(exit_code)][exit] if the value is an [`Err`] or [`None`], after printing the
    /// passed message to [`stderr`].
    ///
    /// [`stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
    /// [exit]: https://doc.rust-lang.org/std/process/fn.exit.html
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```{.should_panic}
    /// # use die::Die;
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.die_code("strange", 3); // prints `strange` to stderr then exits with code 3
    /// ```
    /// ```{.should_panic}
    /// # use die::Die;
    /// let x: Option<u32> = None;
    /// x.die_code("strange", 3); // prints `strange` to stderr then exits with code 3
    /// ```
    fn die_code(self, msg: &str, exit_code: i32) -> T;
}

impl<T, E> Die<T> for Result<T, E> {
    #[inline]
    fn die(self, msg: &str) -> T {
        self.die_code(msg, DEFAULT_EXIT_CODE)
    }
    #[inline]
    fn die_code(self, msg: &str, exit_code: i32) -> T {
        match self {
            Ok(t) => t,
            Err(_) => PrintExit::print_exit(&(exit_code, msg)),
        }
    }
}

impl<T> Die<T> for Option<T> {
    #[inline]
    fn die(self, msg: &str) -> T {
        self.die_code(msg, DEFAULT_EXIT_CODE)
    }
    #[inline]
    fn die_code(self, msg: &str, exit_code: i32) -> T {
        match self {
            Some(t) => t,
            None => PrintExit::print_exit(&(exit_code, msg)),
        }
    }
}

pub trait PrintExit {
    #[inline]
    fn print_exit(&self) -> !;
}

impl PrintExit for i32 {
    #[inline]
    fn print_exit(&self) -> ! {
        process::exit(*self)
    }
}

impl PrintExit for &str {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(DEFAULT_EXIT_CODE)
    }
}

impl PrintExit for String {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(DEFAULT_EXIT_CODE)
    }
}

impl PrintExit for (i32, &str) {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self.1);
        process::exit(self.0)
    }
}

impl PrintExit for (i32, String) {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self.1);
        process::exit(self.0)
    }
}

impl PrintExit for (&str, i32) {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self.0);
        process::exit(self.1)
    }
}

impl PrintExit for (String, i32) {
    #[inline]
    fn print_exit(&self) -> ! {
        eprintln!("{}", self.0);
        process::exit(self.1)
    }
}
