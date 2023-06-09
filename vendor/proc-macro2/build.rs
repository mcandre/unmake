// rustc-cfg emitted by the build script:
//
// "use_proc_macro"
//     Link to extern crate proc_macro. Available on any compiler and any target
//     except wasm32. Requires "proc-macro" Cargo cfg to be enabled (default is
//     enabled). On wasm32 we never link to proc_macro even if "proc-macro" cfg
//     is enabled.
//
// "wrap_proc_macro"
//     Wrap types from libproc_macro rather than polyfilling the whole API.
//     Enabled on rustc 1.29+ as long as procmacro2_semver_exempt is not set,
//     because we can't emulate the unstable API without emulating everything
//     else. Also enabled unconditionally on nightly, in which case the
//     procmacro2_semver_exempt surface area is implemented by using the
//     nightly-only proc_macro API.
//
// "hygiene"
//    Enable Span::mixed_site() and non-dummy behavior of Span::resolved_at
//    and Span::located_at. Enabled on Rust 1.45+.
//
// "proc_macro_span"
//     Enable non-dummy behavior of Span::start and Span::end methods which
//     requires an unstable compiler feature. Enabled when building with
//     nightly, unless `-Z allow-feature` in RUSTFLAGS disallows unstable
//     features.
//
// "super_unstable"
//     Implement the semver exempt API in terms of the nightly-only proc_macro
//     API. Enabled when using procmacro2_semver_exempt on a nightly compiler.
//
// "span_locations"
//     Provide methods Span::start and Span::end which give the line/column
//     location of a token. Enabled by procmacro2_semver_exempt or the
//     "span-locations" Cargo cfg. This is behind a cfg because tracking
//     location inside spans is a performance hit.
//
// "is_available"
//     Use proc_macro::is_available() to detect if the proc macro API is
//     available or needs to be polyfilled instead of trying to use the proc
//     macro API and catching a panic if it isn't available. Enabled on Rust
//     1.57+.

use std::env;
use std::process::{self, Command};
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let version = match rustc_version() {
        Some(version) => version,
        None => return,
    };

    if version.minor < 31 {
        eprintln!("Minimum supported rustc version is 1.31");
        process::exit(1);
    }

    let docs_rs = env::var_os("DOCS_RS").is_some();
    let semver_exempt = cfg!(procmacro2_semver_exempt) || docs_rs;
    if semver_exempt {
        // https://github.com/dtolnay/proc-macro2/issues/147
        println!("cargo:rustc-cfg=procmacro2_semver_exempt");
    }

    if semver_exempt || cfg!(feature = "span-locations") {
        println!("cargo:rustc-cfg=span_locations");
    }

    if version.minor < 32 {
        println!("cargo:rustc-cfg=no_libprocmacro_unwind_safe");
    }

    if version.minor < 34 {
        println!("cargo:rustc-cfg=no_try_from");
    }

    if version.minor < 39 {
        println!("cargo:rustc-cfg=no_bind_by_move_pattern_guard");
    }

    if version.minor < 44 {
        println!("cargo:rustc-cfg=no_lexerror_display");
    }

    if version.minor < 45 {
        println!("cargo:rustc-cfg=no_hygiene");
    }

    if version.minor < 47 {
        println!("cargo:rustc-cfg=no_ident_new_raw");
    }

    if version.minor < 54 {
        println!("cargo:rustc-cfg=no_literal_from_str");
    }

    if version.minor < 55 {
        println!("cargo:rustc-cfg=no_group_open_close");
    }

    if version.minor < 57 {
        println!("cargo:rustc-cfg=no_is_available");
    }

    if version.minor < 66 {
        println!("cargo:rustc-cfg=no_source_text");
    }

    let target = env::var("TARGET").unwrap();
    if !enable_use_proc_macro(&target) {
        return;
    }

    println!("cargo:rustc-cfg=use_proc_macro");

    if version.nightly || !semver_exempt {
        println!("cargo:rustc-cfg=wrap_proc_macro");
    }

    if version.nightly
        && feature_allowed("proc_macro_span")
        && feature_allowed("proc_macro_span_shrink")
    {
        println!("cargo:rustc-cfg=proc_macro_span");
    }

    if semver_exempt && version.nightly {
        println!("cargo:rustc-cfg=super_unstable");
    }
}

fn enable_use_proc_macro(target: &str) -> bool {
    // wasm targets don't have the `proc_macro` crate, disable this feature.
    if target.contains("wasm32") {
        return false;
    }

    // Otherwise, only enable it if our feature is actually enabled.
    cfg!(feature = "proc-macro")
}

struct RustcVersion {
    minor: u32,
    nightly: bool,
}

fn rustc_version() -> Option<RustcVersion> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let nightly = version.contains("nightly") || version.contains("dev");
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    Some(RustcVersion { minor, nightly })
}

fn feature_allowed(feature: &str) -> bool {
    // Recognized formats:
    //
    //     -Z allow-features=feature1,feature2
    //
    //     -Zallow-features=feature1,feature2

    let flags_var;
    let flags_var_string;
    let flags = if let Some(encoded_rustflags) = env::var_os("CARGO_ENCODED_RUSTFLAGS") {
        flags_var = encoded_rustflags;
        flags_var_string = flags_var.to_string_lossy();
        flags_var_string.split('\x1f')
    } else {
        return true;
    };

    for mut flag in flags {
        if flag.starts_with("-Z") {
            flag = &flag["-Z".len()..];
        }
        if flag.starts_with("allow-features=") {
            flag = &flag["allow-features=".len()..];
            return flag.split(',').any(|allowed| allowed == feature);
        }
    }

    // No allow-features= flag, allowed by default.
    true
}
