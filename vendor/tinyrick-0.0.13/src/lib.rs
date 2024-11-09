//! Common build patterns

extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

/// Cargo toggle
pub static FEATURE: &str = "letmeout";

/// Environment name controlling verbosity
pub static VERBOSE_ENVIRONMENT_NAME: &str = "VERBOSE";

lazy_static::lazy_static! {
    static ref DEPENDENCY_CACHE_MUTEX: Mutex<HashMap<fn(), bool>> = Mutex::new(HashMap::new());

    pub static ref PHONY_TASK_MUTEX: Mutex<Vec<fn()>> = Mutex::new(Vec::new());
}

/// Query common host binary suffix
pub fn binary_suffix() -> String {
    if cfg!(windows) {
        return ".exe".to_string();
    }

    String::new()
}

/// Declare a dependency on a task that may panic
pub fn deps(task: fn()) {
    let phony: bool = PHONY_TASK_MUTEX.lock().unwrap().contains(&task);
    let has_run: bool = DEPENDENCY_CACHE_MUTEX.lock().unwrap().contains_key(&task);

    if phony || !has_run {
        task();
        DEPENDENCY_CACHE_MUTEX.lock().unwrap().insert(task, true);
    }
}

/// Declare tasks with no obviously cacheable artifacts.
#[macro_export]
macro_rules! phony {
    ($t : expr) => {
        {
            tinyrick::PHONY_TASK_MUTEX
            .lock()
            .unwrap()
            .push($t);
        }
    };
    ($t : expr, $($u : expr),*) => {
        {
            let ref mut phony_tasks = tinyrick::PHONY_TASK_MUTEX
            .lock()
            .unwrap();

            phony_tasks.push($t);
            $( phony_tasks.push($u); )*
        }
    };
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the command object.
#[macro_export]
macro_rules! exec_mut_with_arguments {
    ($p : expr, $a : expr) => {{
        use std::env::var;
        use std::process::Command;

        if var(tinyrick::VERBOSE_ENVIRONMENT_NAME).is_ok() {
            println!("{} {}", $p, $a.join(" "));
        }

        Command::new($p).args($a)
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program. Can also accept CLI arguments collection.
/// Returns the command object.
#[macro_export]
macro_rules! exec_mut {
    ($p : expr) => {{
        let args: &[&str] = &[];
        tinyrick::exec_mut_with_arguments!($p, args)
    }};
    ($p : expr, $a : expr) => {{
        tinyrick::exec_mut_with_arguments!($p, $a)
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the output object.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec_output {
    ($p : expr) => {{
        tinyrick::exec_mut!($p).output().unwrap()
    }};
    ($p : expr, $a : expr) => {{
        tinyrick::exec_mut!($p, $a).output().unwrap()
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the stdout stream.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec_stdout {
    ($p : expr) => {{
        tinyrick::exec_output!($p).stdout
    }};
    ($p : expr, $a : expr) => {{
        tinyrick::exec_output!($p, $a).stdout
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the stdout stream.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec_stderr {
    ($p : expr) => {{
        tinyrick::exec_output!($p).stderr
    }};
    ($p : expr, $a : expr) => {{
        tinyrick::exec_output!($p, $a).stderr
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the complete stdout string.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec_stdout_utf8 {
    ($p : expr) => {{
        String::from_utf8(tinyrick::exec_stdout!($p)).unwrap()
    }};
    ($p : expr, $a : expr) => {{
        String::from_utf8(tinyrick::exec_stdout!($p, $a)).unwrap()
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the complete stderr string.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec_stderr_utf8 {
    ($p : expr) => {{
        String::from_utf8(tinyrick::exec_stderr!($p)).unwrap()
    }};
    ($p : expr, $a : expr) => {{
        String::from_utf8(tinyrick::exec_stderr!($p, $a)).unwrap()
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Returns the status.
/// Panics if the command could not run to completion.
#[macro_export]
macro_rules! exec_status {
    ($p : expr) => {{
        tinyrick::exec_mut!($p).status().unwrap()
    }};
    ($p : expr, $a : expr) => {{
        tinyrick::exec_mut!($p, $a).status().unwrap()
    }};
}

/// Hey genius, avoid executing commands whenever possible! Look for Rust libraries instead.
///
/// Executes the given program with the given arguments.
/// Panics if the command exits with a failure status.
#[macro_export]
macro_rules! exec {
    ($p : expr) => {{
        assert!(tinyrick::exec_status!($p).success());
    }};
    ($p : expr, $a : expr) => {{
        assert!(tinyrick::exec_status!($p, $a).success())
    }};
}

/// Show registered tasks
#[macro_export]
macro_rules! list_tasks {
    ($t : expr) => {
        {
            use std::process;

            println!("Registered tasks:\n");
            println!("* {}", stringify!($t));
            process::exit(0);
        }
    };
    ($t : expr, $($u : expr),*) => {
        {
            use std::process;

            println!("Registered tasks:\n");
            println!("* {}", stringify!($t));
            $(println!("* {}", stringify!($u));)*
            process::exit(0);
        }
    };
}

/// Register tasks with CLI entrypoint.
/// The first entry is the default task,
/// When no tasks are named in CLI arguments.
#[macro_export]
macro_rules! wubba_lubba_dub_dub {
    ($d : expr ; $($t : expr),*) => {
        use std::env;
        use std::process;

        let arguments: Vec<String> = env::args().collect();

        let task_names: Vec<&str> = arguments
            .iter()
            .skip(1)
            .map(String::as_str)
            .collect();

        if task_names.is_empty() {
            $d();
            process::exit(0);
        }

        for task_name in task_names {
            match task_name {
                "-l" => tinyrick::list_tasks!($d $(, $t)*),
                "--list" => tinyrick::list_tasks!($d $(, $t)*),
                stringify!($d) => $d(),
                $(stringify!($t) => $t(),)*
                _ => panic!("Unknown task {}", task_name)
            }
        }
    };
}
