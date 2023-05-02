//! warnings generates makefile recommendations.

use ast;
use inspect;
use std::fmt;

/// Policy implements a linter check.
pub type Policy = fn(&inspect::Metadata, &[ast::Gem]) -> Vec<Warning>;

pub static UB_LATE_POSIX_MARKER: &str =
    "UB_LATE_POSIX_MARKER: a .POSIX: special target rule must be either the first non-comment line, or absent";

/// check_ub_late_posix_marker reports UB_LATE_POSIX_MARKER violations.
fn check_ub_late_posix_marker(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .enumerate()
        .filter(|(i, e)| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs: _ } => i > &0 && ts == &vec![".POSIX".to_string()],
            _ => false,
        })
        .map(|(_, e)| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: UB_LATE_POSIX_MARKER.to_string(),
        })
        .collect()
}

pub static UB_AMBIGUOUS_INCLUDE: &str =
    "UB_AMBIGUOUS_INCLUDE: unclear whether include line or macro definition";

/// check_ub_ambiguous_include reports UB_AMBIGUOUS_INCLUDE violations.
fn check_ub_ambiguous_include(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::In { ps } => ps.iter().any(|e2| e2.starts_with('=')),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: UB_AMBIGUOUS_INCLUDE.to_string(),
        })
        .collect()
}

pub static UB_SHELL_MACRO: &str = "UB_SHELL_MACRO: do not use or modify SHELL macro";

/// check_ub_shell_macro reports UB_SHELL_MACRO violations.
fn check_ub_shell_macro(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Mc { n, v: _ } => n == &"SHELL".to_string(),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: UB_SHELL_MACRO.to_string(),
        })
        .collect()
}

/// Warning models a linter recommendation.
#[derive(Debug, PartialEq)]
pub struct Warning {
    /// path denotes an offending file path.
    pub path: String,

    /// line denotes the location of the relevant code section to enhance.
    pub line: usize,

    /// policy denotes the nature of the recommendation.
    pub policy: String,
}

impl Warning {
    /// new constructs a Warning.
    pub fn new() -> Warning {
        Warning {
            path: String::new(),
            line: 0,
            policy: String::new(),
        }
    }
}

impl Default for Warning {
    /// default generates a basic Warning.
    fn default() -> Self {
        Warning::new()
    }
}

impl fmt::Display for Warning {
    /// fmt renders a Warning for console use.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "warning: {}:{} {}", self.path, self.line, self.policy,)
    }
}

pub static MAKEFILE_PRECEDENCE: &str =
    "MAKEFILE_PRECEDENCE: lowercase Makefile to makefile for launch speed";

/// check_makefile_precedence reports MAKEFILE_PRECEDENCE violations.
fn check_makefile_precedence(metadata: &inspect::Metadata, _: &[ast::Gem]) -> Vec<Warning> {
    if metadata.filename == "Makefile" {
        return vec![Warning {
            path: metadata.path.clone(),
            line: 0,
            policy: MAKEFILE_PRECEDENCE.to_string(),
        }];
    }

    Vec::new()
}

pub static WAIT_NOP: &str = "WAIT_NOP: .WAIT as a target has no effect";

/// check_makefile_precedence reports WAIT_NOP violations.
fn check_wait_nop(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs: _ } => ts.contains(&".WAIT".to_string()),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: WAIT_NOP.to_string(),
        })
        .collect()
}

/// lint generates warnings for a makefile.
pub fn lint(metadata: inspect::Metadata, makefile: &str) -> Result<Vec<Warning>, String> {
    let gems: Vec<ast::Gem> = ast::parse_posix(&metadata.path, makefile)?.ns;
    let mut warnings: Vec<Warning> = Vec::new();

    let policies: Vec<Policy> = vec![
        check_ub_late_posix_marker,
        check_ub_ambiguous_include,
        check_ub_shell_macro,
        check_makefile_precedence,
        check_wait_nop,
    ];

    for policy in policies {
        warnings.extend(policy(&metadata, &gems));
    }

    Ok(warnings)
}

/// mock_md constructs simulated Metadata for a hypothetical path.
pub fn mock_md(pth: &str) -> inspect::Metadata {
    inspect::Metadata {
        path: pth.to_string(),
        filename: pth.to_string(),
        is_makefile: true,
        build_system: "make".to_string(),
        is_machine_generated: false,
    }
}

#[test]
pub fn test_line_numbers() {
    assert_eq!(
        lint(mock_md("-"), "PKG=curl\n.POSIX:\n").unwrap(),
        vec![Warning {
            path: "-".to_string(),
            line: 2,
            policy: UB_LATE_POSIX_MARKER.to_string(),
        },]
    );
}

#[test]
pub fn test_ub_warnings() {
    assert_eq!(
        lint(mock_md("-"), "PKG=curl\n.POSIX:\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![UB_LATE_POSIX_MARKER]
    );

    assert_eq!(
        lint(mock_md("-"), "PKG=curl\n.POSIX:\n.POSIX:\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![UB_LATE_POSIX_MARKER, UB_LATE_POSIX_MARKER]
    );

    assert_eq!(
        lint(mock_md("-"), "# strict posix\n.POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), "PKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), "\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), "")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), ".POSIX:\ninclude =foo.mk\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![UB_AMBIGUOUS_INCLUDE]
    );

    assert_eq!(
        lint(mock_md("-"), ".POSIX:\ninclude=foo.mk\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("-"), ".POSIX:\nSHELL ?= sh\nSHELL = sh\nSHELL ::= sh\nSHELL :::= sh\nSHELL += sh\nSHELL != sh\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![
            UB_SHELL_MACRO,
            UB_SHELL_MACRO,
            UB_SHELL_MACRO,
            UB_SHELL_MACRO,
            UB_SHELL_MACRO,
            UB_SHELL_MACRO,
        ]
    );
}

#[test]
pub fn test_makefile_precedence() {
    assert_eq!(
        lint(mock_md("Makefile"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![MAKEFILE_PRECEDENCE]
    );

    assert_eq!(
        lint(mock_md("makefile"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("foo.mk"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("foo.Makefile"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    assert_eq!(
        lint(mock_md("foo.makefile"), ".POSIX:\nPKG=curl\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
}

#[test]
pub fn test_wait_nop() {
    assert_eq!(
        lint(mock_md("-"), ".POSIX:\n.WAIT:\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        vec![WAIT_NOP]
    );

    assert_eq!(
        lint(mock_md("-"), ".POSIX:\ntest: test-1 .WAIT test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
}
