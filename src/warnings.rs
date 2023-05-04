//! warnings generates makefile recommendations.

use ast;
use inspect;
use std::collections::HashSet;
use std::fmt;

lazy_static::lazy_static! {
    /// WD_COMMANDS collects common commands for modifying a shell's current working directory.
    pub static ref WD_COMMANDS: Vec<String> = vec![
        "cd".to_string(),
        "pushd".to_string(),
        "popd".to_string(),
    ];

    /// LOWER_CONVENTIONAL_PHONY_TARGETS_PATTERN matches common artifactless target names,
    /// specified in lowercase.
    pub static ref LOWER_CONVENTIONAL_PHONY_TARGETS_PATTERN: regex::Regex = regex::Regex::new(
        "^all|lint|install|uninstall|publish|(test.*)|(clean.*)$"
    ).unwrap();

    /// COMMAND_PREFIX_PATTERN matches commands with prefixes.
    pub static ref COMMAND_PREFIX_PATTERN: regex::Regex = regex::Regex::new(r"^(?P<prefix>[-+@]+)").unwrap();

    /// BLANK_COMMAND_PATTERN matches empty commands.
    ///
    /// Empty commands are distinct from a rule without commands.
    pub static ref BLANK_COMMAND_PATTERN: regex::Regex = regex::Regex::new(r"^[-+@]+\s*$").unwrap();

    /// WHITESPACE_LEADING_COMMAND_PATTERN matches commands that start with whitespace.
    pub static ref WHITESPACE_LEADING_COMMAND_PATTERN: regex::Regex = regex::Regex::new(r"^[-+@]*\s+").unwrap();

    /// POLICIES collects the set of available high level makefile checks.
    pub static ref POLICIES: Vec<Policy> = vec![
        check_ub_late_posix_marker,
        check_ub_ambiguous_include,
        check_ub_makeflags_assignment,
        check_ub_shell_macro,
        check_strict_posix,
        check_implementation_defined_target,
        check_makefile_precedence,
        check_curdir_assignment_nop,
        check_wd_nop,
        check_wait_nop,
        check_phony_nop,
        check_redundant_notparallel_wait,
        check_redundant_silent_at,
        check_redundant_ignore_minus,
        check_global_ignore,
        check_simplify_at,
        check_simplify_minus,
        check_command_comment,
        check_phony_target,
        check_repeated_command_prefix,
        check_blank_command,
        check_whitespace_leading_command,
        check_no_rules,
        check_rule_all,
        check_final_eol,
    ];
}

/// Policy implements a linter check.
pub type Policy = fn(&inspect::Metadata, &[ast::Gem]) -> Vec<Warning>;

pub static UB_LATE_POSIX_MARKER: &str =
    "UB_LATE_POSIX_MARKER: the special rule \".POSIX:\" should be the first uncommented instruction in POSIX makefiles, or else absent from *.include.mk files";

/// check_ub_late_posix_marker reports UB_LATE_POSIX_MARKER violations.
fn check_ub_late_posix_marker(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .enumerate()
        .filter(|(i, e)| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs: _ } => {
                (metadata.is_include_file || i > &0) && ts == &vec![".POSIX".to_string()]
            }
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

pub static UB_MAKEFLAGS_ASSIGNMENT: &str = "UB_MAKEFLAGS_MACRO: do not modify MAKEFLAGS macro";

/// check_ub_makeflags_assignment reports UB_MAKEFLAGS_ASSIGNMENT violations.
fn check_ub_makeflags_assignment(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Mc { n, v: _ } => n == &"MAKEFLAGS".to_string(),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: UB_MAKEFLAGS_ASSIGNMENT.to_string(),
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
        write!(f, "warning: {}:", self.path)?;

        if self.line > 0 {
            write!(f, "{}:", self.line)?;
        }

        write!(f, " {}", self.policy)
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

pub static CURDIR_ASSIGNMENT_NOP: &str =
    "CURDIR_ASSIGNMENT_NOP: CURDIR assignment does not change the make working directory";

/// check_curdir_assignment_nop reports CURDIR_ASSIGNMENT_NOP violations.
fn check_curdir_assignment_nop(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Mc { n, v: _ } => n == &"CURDIR".to_string(),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: CURDIR_ASSIGNMENT_NOP.to_string(),
        })
        .collect()
}

pub static WD_NOP: &str =
    "WD_NOP: change directory commands may not persist across successive commands or rules";

/// check_wd_nop reports WD_NOP violations.
fn check_wd_nop(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts: _, cs } => cs.iter().any(|e2| {
                WD_COMMANDS.contains(&e2.split_whitespace().next().unwrap_or("").to_string())
            }),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: WD_NOP.to_string(),
        })
        .collect()
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

pub static PHONY_NOP: &str = "PHONY_NOP: empty .PHONY has no effect";

/// check_phony_nop reports PHONY_NOP violations.
fn check_phony_nop(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps, ts, cs: _ } => ts.contains(&".PHONY".to_string()) && ps.is_empty(),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: PHONY_NOP.to_string(),
        })
        .collect()
}

pub static REDUNDANT_NOTPARALLEL_WAIT: &str =
    "REDUNDANT_NOTPARALLEL_WAIT: .NOTPARALLEL with .WAIT is redundant and superfluous";

/// check_redundant_notparallel_wait reports REDUNDANT_NOTPARALLEL_WAIT violations.
fn check_redundant_notparallel_wait(
    metadata: &inspect::Metadata,
    gems: &[ast::Gem],
) -> Vec<Warning> {
    let has_notparallel: bool = gems.iter().any(|e| match &e.n {
        ast::Ore::Ru { ps: _, ts, cs: _ } => ts.contains(&".NOTPARALLEL".to_string()),
        _ => false,
    });

    if !has_notparallel {
        return Vec::new();
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps, ts: _, cs: _ } => ps.contains(&".WAIT".to_string()),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: REDUNDANT_NOTPARALLEL_WAIT.to_string(),
        })
        .collect()
}

pub static REDUNDANT_SILENT_AT: &str =
    "REDUNDANT_SILENT_AT: .SILENT with @ is redundant and superfluous";

/// check_redundant_silent_at reports REDUNDANT_SILENT_AT violations.
fn check_redundant_silent_at(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    let mut has_global_silence: bool = false;
    let mut marked_silent_targets: HashSet<String> = HashSet::new();

    for gem in gems {
        if let ast::Ore::Ru { ps, ts, cs: _ } = &gem.n {
            if ts.contains(&".SILENT".to_string()) {
                if ps.is_empty() {
                    has_global_silence = true;
                }

                for p in ps {
                    marked_silent_targets.insert(p.clone());
                }
            }
        }
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs } => {
                cs.iter().any(|e2| e2.starts_with('@'))
                    && (has_global_silence
                        || ts.iter().any(|e2| marked_silent_targets.contains(e2)))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: REDUNDANT_SILENT_AT.to_string(),
        })
        .collect()
}

pub static REDUNDANT_IGNORE_MINUS: &str =
    "REDUNDANT_IGNORE_MINUS: .IGNORE with - is redundant and superfluous";

/// check_redundant_ignore_minus reports REDUNDANT_IGNORE_MINUS violations.
fn check_redundant_ignore_minus(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    let mut marked_ignored_targets: HashSet<String> = HashSet::new();
    for gem in gems {
        if let ast::Ore::Ru { ps, ts, cs: _ } = &gem.n {
            if ts.contains(&".IGNORE".to_string()) {
                for p in ps {
                    marked_ignored_targets.insert(p.clone());
                }
            }
        }
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs } => {
                cs.iter().any(|e2| e2.starts_with('-'))
                    && ts.iter().any(|e2| marked_ignored_targets.contains(e2))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: REDUNDANT_IGNORE_MINUS.to_string(),
        })
        .collect()
}

pub static GLOBAL_IGNORE: &str =
    "GLOBAL_IGNORE: .IGNORE without prerequisites may corrupt artifacts";

/// check_global_ignore reports GLOBAL_IGNORE violations.
fn check_global_ignore(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps, ts, cs: _ } => ts.contains(&".IGNORE".to_string()) && ps.is_empty(),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: GLOBAL_IGNORE.to_string(),
        })
        .collect()
}

pub static SIMPLIFY_AT: &str =
    "SIMPLIFY_AT: replace individual at (@) signs with .SILENT target declaration(s)";

/// check_simplify_at reports SIMPLIFY_AT violations.
fn check_simplify_at(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    let mut has_global_silence: bool = false;
    let mut marked_silent_targets: HashSet<String> = HashSet::new();

    for gem in gems {
        if let ast::Ore::Ru { ps, ts, cs: _ } = &gem.n {
            if ts.contains(&".SILENT".to_string()) {
                if ps.is_empty() {
                    has_global_silence = true;
                }

                for p in ps {
                    marked_silent_targets.insert(p.clone());
                }
            }
        }
    }

    if has_global_silence {
        return Vec::new();
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs } => {
                cs.len() > 1
                    && cs.iter().all(|e2| e2.starts_with('@'))
                    && !ts.iter().any(|e2| marked_silent_targets.contains(e2))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: SIMPLIFY_AT.to_string(),
        })
        .collect()
}

pub static SIMPLIFY_MINUS: &str =
    "SIMPLIFY_MINUS: replace individual hyphen-minus (-) signs with .IGNORE target declaration(s)";

/// check_simplify_minus reports SIMPLIFY_MINUS violations.
fn check_simplify_minus(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    let mut has_global_ignore: bool = false;
    let mut marked_ignored_targets: HashSet<String> = HashSet::new();

    for gem in gems {
        if let ast::Ore::Ru { ps, ts, cs: _ } = &gem.n {
            if ts.contains(&".IGNORE".to_string()) {
                if ps.is_empty() {
                    has_global_ignore = true;
                }

                for p in ps {
                    marked_ignored_targets.insert(p.clone());
                }
            }
        }
    }

    if has_global_ignore {
        return Vec::new();
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs } => {
                cs.len() > 1
                    && cs.iter().all(|e2| e2.starts_with('-'))
                    && !ts.iter().any(|e2| marked_ignored_targets.contains(e2))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: SIMPLIFY_MINUS.to_string(),
        })
        .collect()
}

pub static STRICT_POSIX: &str =
    "STRICT_POSIX: lead makefiles with the \".POSIX:\" compliance marker, or else rename include files to *.include.mk";

/// check_strict_posix reports STRICT_POSIX violations.
fn check_strict_posix(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    if metadata.is_include_file {
        return Vec::new();
    }

    let has_strict_posix: bool = gems.iter().any(|e| match &e.n {
        ast::Ore::Ru { ps: _, ts, cs: _ } => ts.contains(&".POSIX".to_string()),
        _ => false,
    });

    if !has_strict_posix {
        return vec![Warning {
            path: metadata.path.clone(),
            line: 1,
            policy: STRICT_POSIX.to_string(),
        }];
    }

    Vec::new()
}

pub static IMPLEMENTATTION_DEFINED_TARGET: &str = "IMPLEMENTATTION_DEFINED_TARGET: non-portable percent (%) or double-quote (\") in target or prerequisite";

/// check_implementation_defined_target reports IMPLEMENTATTION_DEFINED_TARGET violations.
fn check_implementation_defined_target(
    metadata: &inspect::Metadata,
    gems: &[ast::Gem],
) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps, ts, cs: _ } => {
                ps.iter().any(|e2| e2.contains('%') || e2.contains('\"'))
                    || ts.iter().any(|e2| e2.contains('%') || e2.contains('\"'))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: IMPLEMENTATTION_DEFINED_TARGET.to_string(),
        })
        .collect()
}

pub static COMMAND_COMMENT: &str =
    "COMMAND_COMMENT: comment embedded inside commands will forward to the shell interpreter";

/// check_command_comment reports COMMAND_COMMENT violations.
fn check_command_comment(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts: _, cs } => cs.iter().any(|e2| e2.contains('#')),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: COMMAND_COMMENT.to_string(),
        })
        .collect()
}

pub static REPEATED_COMMAND_PREFIX: &str =
    "REPEATED_COMMAND_PREFIX: redundant prefixes are superfluous";

/// check_blank_command reports BLANK_COMMAND violations.
fn check_repeated_command_prefix(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts: _, cs } => cs.iter().any(|e2| {
                if BLANK_COMMAND_PATTERN.is_match(e2) {
                    return false;
                }

                let prefix: &str = COMMAND_PREFIX_PATTERN
                    .captures(e2)
                    .and_then(|e3| e3.name("prefix"))
                    .map(|e3| e3.as_str())
                    .unwrap_or("");

                prefix.matches('@').count() > 1
                    || prefix.matches('+').count() > 1
                    || prefix.matches('-').count() > 1
            }),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: REPEATED_COMMAND_PREFIX.to_string(),
        })
        .collect()
}

pub static BLANK_COMMAND: &str =
    "BLANK_COMMAND: indeterminate behavior when empty commands are sent to assorted shell interpreters";

/// check_blank_command reports BLANK_COMMAND violations.
fn check_blank_command(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts: _, cs } => {
                cs.iter().any(|e2| BLANK_COMMAND_PATTERN.is_match(e2))
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: BLANK_COMMAND.to_string(),
        })
        .collect()
}

pub static WHITESPACE_LEADING_COMMAND: &str =
    "WHITESPACE_LEADING_COMMAND: questionable whitespace detected at the start of a command";

/// check_whitespace_leading_command reports WHITESPACE_LEADING_COMMAND violations.
fn check_whitespace_leading_command(
    metadata: &inspect::Metadata,
    gems: &[ast::Gem],
) -> Vec<Warning> {
    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts: _, cs } => cs
                .iter()
                .any(|e2| WHITESPACE_LEADING_COMMAND_PATTERN.is_match(e2)),
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: WHITESPACE_LEADING_COMMAND.to_string(),
        })
        .collect()
}

pub static MISSING_FINAL_EOL: &str =
    "MISSING_FINAL_EOL: UNIX text files may process poorly without a final LF";

/// check_final_eol reports MISSING_FINAL_EOL violations.
fn check_final_eol(metadata: &inspect::Metadata, _: &[ast::Gem]) -> Vec<Warning> {
    if !metadata.is_empty && !metadata.has_final_eol {
        return vec![Warning {
            path: metadata.path.clone(),
            line: metadata.lines,
            policy: MISSING_FINAL_EOL.to_string(),
        }];
    }

    Vec::new()
}

pub static PHONY_TARGET: &str = "PHONY_TARGET: mark common artifactless rules as .PHONY";

/// check_phony_target reports PHONY_TARGET violations.
fn check_phony_target(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    let mut marked_phony_targets: HashSet<String> = HashSet::new();
    for gem in gems {
        if let ast::Ore::Ru { ps, ts, cs: _ } = &gem.n {
            if ts.contains(&".PHONY".to_string()) {
                for p in ps {
                    marked_phony_targets.insert(p.clone());
                }
            }
        }
    }

    gems.iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs }
                if !ts.iter().any(|e2| ast::SPECIAL_TARGETS.contains(e2))
                    && ts.iter().any(|e2| !marked_phony_targets.contains(e2)) =>
            {
                ts.iter().any(|e2| {
                    LOWER_CONVENTIONAL_PHONY_TARGETS_PATTERN.is_match(e2.to_lowercase().as_str())
                }) || cs.is_empty()
            }
            _ => false,
        })
        .map(|e| Warning {
            path: metadata.path.clone(),
            line: e.l,
            policy: PHONY_TARGET.to_string(),
        })
        .collect()
}

pub static NO_RULES: &str =
    "NO_RULES: declare at least one non-special rule, or else rename to *.include.mk";

/// check_no_rules reports NO_RULES violations.
fn check_no_rules(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    if metadata.is_include_file {
        return Vec::new();
    }

    let has_nonspecial_rule: bool = !gems
        .iter()
        .filter(|e| match &e.n {
            ast::Ore::Ru { ps: _, ts, cs: _ } => {
                ts.iter().any(|e2| !ast::SPECIAL_TARGETS.contains(e2))
            }
            _ => false,
        })
        .collect::<Vec<&ast::Gem>>()
        .is_empty();

    if !has_nonspecial_rule {
        return vec![Warning {
            path: metadata.path.clone(),
            line: 0,
            policy: NO_RULES.to_string(),
        }];
    }

    Vec::new()
}

pub static RULE_ALL: &str =
    "RULE_ALL: makefiles conventionally name the first non-special, default rule \"all\"";

/// check_rule_all reports RULE_ALL violations.
fn check_rule_all(metadata: &inspect::Metadata, gems: &[ast::Gem]) -> Vec<Warning> {
    if metadata.is_include_file {
        return Vec::new();
    }

    let mut first_nonspecial_target: &String = &String::new();
    let mut found_nonspecial_target: bool = false;

    for gem in gems {
        match &gem.n {
            ast::Ore::Ru { ps: _, ts, cs: _ }
                if !ts.is_empty() && ts.iter().all(|e2| !ast::SPECIAL_TARGETS.contains(e2)) =>
            {
                found_nonspecial_target = true;
                first_nonspecial_target = ts.first().unwrap();
                break;
            }
            _ => (),
        }
    }

    if found_nonspecial_target && first_nonspecial_target != &"all".to_string() {
        return vec![Warning {
            path: metadata.path.clone(),
            line: 0,
            policy: RULE_ALL.to_string(),
        }];
    }

    Vec::new()
}

/// lint generates warnings for a makefile.
pub fn lint(metadata: &inspect::Metadata, makefile: &str) -> Result<Vec<Warning>, String> {
    let gems: Vec<ast::Gem> = ast::parse_posix(&metadata.path, makefile)?.ns;
    let mut warnings: Vec<Warning> = Vec::new();

    for policy in POLICIES.iter() {
        warnings.extend(policy(metadata, &gems));
    }

    Ok(warnings)
}

/// mock_md constructs simulated Metadata for a hypothetical path.
///
/// Assume a lintable POSIX makefile.
///
/// Certain fields are given dummy values.
pub fn mock_md(pth: &str) -> inspect::Metadata {
    inspect::Metadata {
        path: pth.to_string(),
        filename: pth.to_string(),
        is_makefile: true,
        build_system: "make".to_string(),
        is_machine_generated: false,
        is_include_file: false,
        is_empty: true,
        lines: 0,
        has_final_eol: false,
    }
}

#[test]
pub fn test_line_numbers() {
    let md: inspect::Metadata = mock_md("-");

    assert_eq!(
        check_ub_late_posix_marker(
            &md,
            &ast::parse_posix(md.path.as_str(), "PKG=curl\n.POSIX:\n")
                .unwrap()
                .ns
        ),
        vec![Warning {
            path: "-".to_string(),
            line: 2,
            policy: UB_LATE_POSIX_MARKER.to_string(),
        },]
    );
}

#[test]
pub fn test_ub_warnings() {
    assert!(lint(&mock_md("-"), "PKG=curl\n.POSIX:\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_LATE_POSIX_MARKER.to_string()));

    assert!(!lint(&mock_md("-"), "# strict posix\n.POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_LATE_POSIX_MARKER.to_string()));

    assert!(!lint(&mock_md("-"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_LATE_POSIX_MARKER.to_string()));

    let mut md_include = mock_md("provision.include.mk");
    md_include.is_include_file = true;

    assert!(lint(&md_include, ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_LATE_POSIX_MARKER.to_string()));

    assert!(!lint(&md_include, "PKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_LATE_POSIX_MARKER.to_string()));

    assert!(lint(&mock_md("-"), ".POSIX:\ninclude =foo.mk\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_AMBIGUOUS_INCLUDE.to_string()));

    assert!(!lint(&mock_md("-"), ".POSIX:\ninclude=foo.mk\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&UB_AMBIGUOUS_INCLUDE.to_string()));

    assert!(
        lint(&mock_md("-"), ".POSIX:\nMAKEFLAGS ?= -j\nMAKEFLAGS = -j\nMAKEFLAGS ::= -j\nMAKEFLAGS :::= -j\nMAKEFLAGS += -j\nMAKEFLAGS != echo \"-j\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>().contains(&UB_MAKEFLAGS_ASSIGNMENT.to_string()));

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nSHELL ?= sh\nSHELL = sh\nSHELL ::= sh\nSHELL :::= sh\nSHELL += sh\nSHELL != sh\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&UB_SHELL_MACRO.to_string()));
}

#[test]
pub fn test_strict_posix() {
    let md_stdin: inspect::Metadata = mock_md("-");

    assert!(lint(&md_stdin, "PKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&STRICT_POSIX.to_string()));

    assert!(!lint(&md_stdin, ".POSIX:\nPKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&STRICT_POSIX.to_string()));

    let mut md_sys: inspect::Metadata = mock_md("sys.mk");
    md_sys.is_include_file = true;

    assert!(!lint(&md_sys, "PKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&STRICT_POSIX.to_string()));

    let mut md_include_mk: inspect::Metadata = mock_md("foo.include.mk");
    md_include_mk.is_include_file = true;

    assert!(!lint(&md_include_mk, "PKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&STRICT_POSIX.to_string()));
}

#[test]
pub fn test_makefile_precedence() {
    assert!(lint(&mock_md("Makefile"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MAKEFILE_PRECEDENCE.to_string()));

    assert!(!lint(&mock_md("makefile"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MAKEFILE_PRECEDENCE.to_string()));

    assert!(!lint(&mock_md("foo.mk"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MAKEFILE_PRECEDENCE.to_string()));

    assert!(!lint(&mock_md("foo.Makefile"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MAKEFILE_PRECEDENCE.to_string()));

    assert!(!lint(&mock_md("foo.makefile"), ".POSIX:\nPKG=curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MAKEFILE_PRECEDENCE.to_string()));
}

#[test]
pub fn test_curdir_assignment_nop() {
    assert!(lint(&mock_md("-"), ".POSIX:\nCURDIR = build\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&CURDIR_ASSIGNMENT_NOP.to_string()));

    assert!(!lint(&mock_md("-"), ".POSIX:\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&CURDIR_ASSIGNMENT_NOP.to_string()));
}

#[test]
pub fn test_wd_nop() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall:\n\tcd foo\n\n\tpushd bar\n\tpopd\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&WD_NOP.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall:\n\ttar -C foo czvf foo.tgz .\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&WD_NOP.to_string()));
}

#[test]
pub fn test_wait_nop() {
    assert!(lint(&mock_md("-"), ".POSIX:\n.WAIT:\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WAIT_NOP.to_string()));

    assert!(
        !lint(&mock_md("-"), ".POSIX:\n.PHONY: test test-1 test-2\ntest: test-1 .WAIT test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>().contains(&WAIT_NOP.to_string()));
}

#[test]
pub fn test_phony_nop() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY:\nfoo: foo.c\n\tgcc -o foo foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_NOP.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\techo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_NOP.to_string()));
}

#[test]
pub fn test_redundant_nonparallel_wait() {
    assert!(
        lint(&mock_md("-"), ".POSIX:\n.NOTPARALLEL:\n.PHONY: test test-1 test-2\ntest: test-1 .WAIT test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>().contains(&REDUNDANT_NOTPARALLEL_WAIT.to_string()));

    assert!(
        !lint(&mock_md("-"), ".POSIX:\n.PHONY: test test-1 test-2\ntest: test-1 .WAIT test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>().contains(&REDUNDANT_NOTPARALLEL_WAIT.to_string()));

    assert!(
        !lint(&mock_md("-"), ".POSIX:\n.NOTPARALLEL:\n.PHONY: test test-1 test-2\ntest: test-1 test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>().contains(&REDUNDANT_NOTPARALLEL_WAIT.to_string()));
}

#[test]
pub fn test_redundant_silent_at() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: lint\n.SILENT:\nlint:\n\t@unmake .\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REDUNDANT_SILENT_AT.to_string()));

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: lint\n.SILENT: lint\nlint:\n\t@unmake .\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REDUNDANT_SILENT_AT.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: lint\n.SILENT: lint\nlint:\n\tunmake .\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REDUNDANT_SILENT_AT.to_string()));

    assert!(
        !lint(&mock_md("-"), ".POSIX:\n.PHONY: lint\nlint:\n\t@unmake .\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&REDUNDANT_SILENT_AT.to_string())
    );
}

#[test]
pub fn test_global_ignore() {
    assert!(lint(&mock_md("-"), ".POSIX:\n.IGNORE:\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&GLOBAL_IGNORE.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: clean\n.IGNORE: clean\nclean:\n\trm -rf bin"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&GLOBAL_IGNORE.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: clean\nclean:\n\t-rm -rf bin"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&GLOBAL_IGNORE.to_string()));
}

#[test]
pub fn test_redundant_ignore_minus() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: clean\n.IGNORE: clean\nclean:\n\t-rm -rf bin\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REDUNDANT_IGNORE_MINUS.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: clean\nclean:\n\t-rm -rf bin\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REDUNDANT_IGNORE_MINUS.to_string()));
}

#[test]
pub fn test_implementation_defined_target() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall: foo%\nfoo%: foo.c\n\tgcc -o foo% foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&IMPLEMENTATTION_DEFINED_TARGET.to_string()));

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall: \"foo\"\n\"foo\": foo.c\n\tgcc -o \"foo\" foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&IMPLEMENTATTION_DEFINED_TARGET.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall: foo\nfoo: foo.c\n\tgcc -o foo foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&IMPLEMENTATTION_DEFINED_TARGET.to_string()));
}

#[test]
pub fn test_command_comment() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nfoo: foo.c\n\t#build foo\n\tgcc -o foo foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&COMMAND_COMMENT.to_string()));

    assert!(
        lint(&mock_md("-"), ".POSIX:\nfoo: foo.c\n\t@#gcc -o foo foo.c\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&COMMAND_COMMENT.to_string())
    );

    assert!(
        lint(&mock_md("-"), ".POSIX:\nfoo: foo.c\n\t-#gcc -o foo foo.c\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&COMMAND_COMMENT.to_string())
    );

    assert!(
        lint(&mock_md("-"), ".POSIX:\nfoo: foo.c\n\t+#gcc -o foo foo.c\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&COMMAND_COMMENT.to_string())
    );

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nfoo: foo.c\n\tgcc \\\n#output file \\\n\t\t-o foo \\\n\t\tfoo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&COMMAND_COMMENT.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\nfoo: foo.c\n#build foo\n\tgcc -o foo foo.c\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&COMMAND_COMMENT.to_string()));
}

#[test]
pub fn test_whitespace_leading_command() {
    assert!(lint(&mock_md("-"), "foo:\n\t gcc -o foo foo.c\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WHITESPACE_LEADING_COMMAND.to_string()));

    assert!(lint(&mock_md("-"), "foo:\n\t\tgcc -o foo foo.c\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WHITESPACE_LEADING_COMMAND.to_string()));

    assert!(lint(&mock_md("-"), "foo:\n\t@+- gcc -o foo foo.c\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WHITESPACE_LEADING_COMMAND.to_string()));

    assert!(!lint(&mock_md("-"), "foo:\n\tgcc -o foo foo.c\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WHITESPACE_LEADING_COMMAND.to_string()));

    assert!(!lint(&mock_md("-"), "foo:\n\t@+-gcc -o foo foo.c\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&WHITESPACE_LEADING_COMMAND.to_string()));

    assert!(
        !lint(&mock_md("-"), "foo:\n\tgcc \\\n\t\t-o \\\n\t\tfoo foo.c\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&WHITESPACE_LEADING_COMMAND.to_string())
    );
}

#[test]
pub fn test_phony_target() {
    assert!(
        lint(&mock_md("-"), ".POSIX:\nall:\n\techo \"Hello World!\"\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&PHONY_TARGET.to_string())
    );

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nlint:;\ninstall:;\nuninstall:;\npublish:;\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_TARGET.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: all\nall:\n\techo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_TARGET.to_string()));

    assert!(
        lint(
            &mock_md("-"),
            ".POSIX:\ntest: test-1 test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n"
        )
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>().contains(&PHONY_TARGET.to_string()));

    assert!(
        !lint(
            &mock_md("-"),
            ".POSIX:\n.PHONY: test test-1 test-2\ntest: test-1 test-2\ntest-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n"
        )
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>().contains(&PHONY_TARGET.to_string()));

    assert!(
        !lint(
            &mock_md("-"),
            ".POSIX:\n.PHONY: test\ntest: test-1 test-2\n.PHONY: test-1\ntest-1:\n\techo \"Hello World!\"\n.PHONY: test-2\ntest-2:\n\techo \"Hi World!\"\n"
        )
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>().contains(&PHONY_TARGET.to_string()));

    assert!(lint(&mock_md("-"), ".POSIX:\nclean:\n\t-rm -rf bin\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&PHONY_TARGET.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: clean\nclean:\n\t-rm -rf bin\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_TARGET.to_string()));

    assert!(
        lint(&mock_md("-"), ".POSIX:\nport: cross-compile archive\n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&PHONY_TARGET.to_string())
    );

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: port\nport: cross-compile archive\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&PHONY_TARGET.to_string()));

    assert!(lint(&mock_md("-"), ".POSIX:\nempty:;\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&PHONY_TARGET.to_string()));

    // Ensure that absent targets do not trigger a PHONY_TARGET warning,
    // unlike some makefile linters in the past.
    assert!(!lint(&mock_md("-"), ".POSIX:\nPKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&PHONY_TARGET.to_string()));
}

#[test]
pub fn test_simplify_at() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nwelcome:\n\t@echo foo\n\t@echo bar\n\t@echo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_AT.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\nwelcome:\n\t@echo foo\n\t@echo bar\n\techo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_AT.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.SILENT: welcome\nwelcome:\n\techo foo\n\techo bar\n\techo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_AT.to_string()));
}

#[test]
pub fn test_simplify_minus() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\nwelcome:\n\t-echo foo\n\t-echo bar\n\t-echo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_MINUS.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\nwelcome:\n\t-echo foo\n\t-echo bar\n\techo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_MINUS.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.IGNORE: welcome\nwelcome:\n\techo foo\n\techo bar\n\techo baz\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&SIMPLIFY_MINUS.to_string()));
}

#[test]
pub fn test_repeated_command_prefix() {
    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\t@@echo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REPEATED_COMMAND_PREFIX.to_string()));

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\t--echo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REPEATED_COMMAND_PREFIX.to_string()));

    assert!(lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\t@-@echo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REPEATED_COMMAND_PREFIX.to_string()));

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\t@+-echo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&REPEATED_COMMAND_PREFIX.to_string()));
}

#[test]
pub fn test_blank_command() {
    assert!(lint(&mock_md("-"), ".POSIX:\n.PHONY: test\ntest:\n\t@\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&BLANK_COMMAND.to_string()));

    assert!(lint(&mock_md("-"), ".POSIX:\n.PHONY: test\ntest:\n\t-\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&BLANK_COMMAND.to_string()));

    assert!(lint(&mock_md("-"), ".POSIX:\n.PHONY: test\ntest:\n\t+\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&BLANK_COMMAND.to_string()));

    assert!(
        lint(&mock_md("-"), ".POSIX:\n.PHONY: test\ntest:\n\t@+- \n")
            .unwrap()
            .into_iter()
            .map(|e| e.policy)
            .collect::<Vec<String>>()
            .contains(&BLANK_COMMAND.to_string())
    );

    assert!(!lint(
        &mock_md("-"),
        ".POSIX:\n.PHONY: test\ntest:\n\techo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&BLANK_COMMAND.to_string()));
}

#[test]
pub fn test_no_rules() {
    let md_stdin: inspect::Metadata = mock_md("-");

    assert!(lint(&md_stdin, ".POSIX:\nPKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&NO_RULES.to_string()));

    let mut md_include: inspect::Metadata = mock_md("foo.include.mk");
    md_include.is_include_file = true;

    assert!(!lint(&md_include, "PKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&NO_RULES.to_string()));

    assert!(!lint(&md_stdin, "all:\n\techo \"Hello World!\"\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&NO_RULES.to_string()));
}

#[test]
pub fn test_rule_all() {
    assert!(lint(&mock_md("-"), "build:\n\techo \"Hello World!\"\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&RULE_ALL.to_string()));

    assert!(!lint(&mock_md("-"), "all:\n\techo \"Hello World!\"\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&RULE_ALL.to_string()));

    assert!(!lint(
        &mock_md("-"),
        "all: build\nbuild:\n\techo \"Hello World!\"\n"
    )
    .unwrap()
    .into_iter()
    .map(|e| e.policy)
    .collect::<Vec<String>>()
    .contains(&RULE_ALL.to_string()));

    assert!(!lint(&mock_md("-"), "PKG = curl\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&RULE_ALL.to_string()));

    let mut md_include = mock_md("foo.include.mk");
    md_include.is_include_file = true;

    assert!(!lint(&md_include, "build:\n\techo \"Hello World!\"\n")
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&RULE_ALL.to_string()));
}

#[test]
pub fn test_final_eol() {
    let mf_pkg: &str = ".POSIX:\nPKG = curl";
    let mut md_pkg: inspect::Metadata = mock_md("-");
    md_pkg.is_empty = &mf_pkg.len() == &0;
    md_pkg.has_final_eol = &mf_pkg.chars().last().unwrap_or(' ') == &'\n';

    assert!(lint(&md_pkg, &mf_pkg)
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MISSING_FINAL_EOL.to_string()));

    let mf_pkg_final_eol: &str = ".POSIX:\nPKG = curl\n";
    let mut md_pkg_final_eol: inspect::Metadata = mock_md("-");
    md_pkg_final_eol.is_empty = &mf_pkg_final_eol.len() == &0;
    md_pkg_final_eol.has_final_eol = &mf_pkg_final_eol.chars().last().unwrap_or(' ') == &'\n';

    assert!(!lint(&md_pkg_final_eol, &mf_pkg_final_eol)
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MISSING_FINAL_EOL.to_string()));

    let mf_empty: &str = "";
    let mut md_empty: inspect::Metadata = mock_md("-");
    md_empty.is_empty = &mf_empty.len() == &0;
    md_empty.has_final_eol = &mf_empty.chars().last().unwrap_or(' ') == &'\n';

    assert!(!lint(&md_empty, &mf_empty)
        .unwrap()
        .into_iter()
        .map(|e| e.policy)
        .collect::<Vec<String>>()
        .contains(&MISSING_FINAL_EOL.to_string()));
}
