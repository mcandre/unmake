//! ast parses makefiles.

extern crate lazy_static;
extern crate peg;
extern crate walkdir;

use self::peg::parser;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};
use std::str::Chars;

/// UPPERCASE_ALPHABETIC matches ASCII uppercase characters.
pub static UPPERCASE_ALPHABETIC: RangeInclusive<char> = 'A'..='Z';

lazy_static::lazy_static! {
    /// SPECIAL_TARGETS collects POSIX special target names.
    pub static ref SPECIAL_TARGETS: HashSet<String> = vec![
            ".POSIX".to_string(),
            ".DEFAULT".to_string(),
            ".IGNORE".to_string(),
            ".NOTPARALLEL".to_string(),
            ".PHONY".to_string(),
            ".PRECIOUS".to_string(),
            ".SCCS_GET".to_string(),
            ".SILENT".to_string(),
            ".SUFFIXES".to_string(),
            ".WAIT".to_string(),
        ]
        .into_iter()
        .collect::<HashSet<String>>();
}

/// is_reserved reports non-special, reserved target names.
pub fn is_reserved(name: &str) -> bool {
    if SPECIAL_TARGETS.contains(name) {
        false
    } else {
        let mut cs: Chars = name.chars();

        match cs.next() {
            Some('.') => match cs.next() {
                Some(c) => c.is_ascii() && c.is_uppercase(),
                _ => false,
            },
            _ => false,
        }
    }
}

#[test]
fn test_target_reservation_status() {
    assert!(is_reserved(".POSIX1"));
    assert!(!is_reserved(".POSIX"));
    assert!(!is_reserved(".WAIT"));

    assert!(is_reserved(".XYZ"));
    assert!(!is_reserved(".xyz"));
    assert!(!is_reserved("XYZ"));

    assert!(!is_reserved("all"));
    assert!(!is_reserved("."));
    assert!(!is_reserved(""));
}

/// Traceable prepares an AST entry to receive updates
/// about parsing location details.
pub trait Traceable {
    /// set_offset applies the given offset.
    fn set_offset(&mut self, offset: usize);

    /// get_offset queries the current offset.
    fn get_offset(&self) -> usize;

    /// set_line applies the given line.
    fn set_line(&mut self, line: usize);

    /// get_line queries the current line.
    fn get_line(&self) -> usize;

    /// update corrects line details.
    fn update(&mut self, index: &HashMap<Range<usize>, usize>) {
        let offset = &self.get_offset();

        for (r, line) in index {
            if r.contains(offset) {
                self.set_line(*line);
                break;
            }
        }
    }
}

/// Node provides convenient behaviors for unit testing.
pub trait Node: Traceable + Debug + PartialEq {}

/// Ore provides raw token information.
///
/// Ores produces by [parse_posix] may receive values as string literals,
/// as originally supplied in the AST. Minimal or no evaluation is performed;
/// The actual value may vary during makefile processing with a live make implementation.
#[derive(Debug, PartialEq)]
pub enum Ore {
    /// Ru models a makefile rule.
    Ru {
        /// ts denotes the target(s) produced by this rule.
        ts: Vec<String>,

        /// ps denotes any prerequisite(s) depended on by this rule.
        ps: Vec<String>,

        /// cs denotes any shell command(s) executed by this rule.
        cs: Vec<String>,
    },

    /// Mc models a makefile macro definition.
    ///
    /// Values
    Mc {
        /// n denotes a name for this macro.
        n: String,

        /// v denotes an unexpanded value for this macro.
        v: String,
    },

    /// In models an include line.
    In {
        /// ps collects the file paths of any further makefile to include.
        ps: Vec<String>,
    },

    /// Ex models a general macro expression.
    Ex {
        /// e denotes an unexpanded macro expression.
        e: String,
    },
}

/// Gem provides tokens enriched
/// with parsing location information.
#[derive(Debug, PartialEq)]
pub struct Gem {
    /// o denotes the offset
    /// of the opening byte
    /// of this AST node from some stream source.
    pub o: usize,

    /// l denotes the opening line
    /// of this AST node from some stream source.
    pub l: usize,

    /// n denotes a content node.
    pub n: Ore,
}

impl Traceable for Gem {
    /// set_offset applies the given offset.
    fn set_offset(&mut self, offset: usize) {
        self.o = offset;
    }

    /// get_offset queries the current offset.
    fn get_offset(&self) -> usize {
        self.o
    }

    /// set_line applies the given line.
    fn set_line(&mut self, line: usize) {
        self.l = line;
    }

    /// get_line queries the current line.
    fn get_line(&self) -> usize {
        self.l
    }
}

/// Mk models a makefile AST.
#[derive(Debug, PartialEq)]
pub struct Mk {
    /// offset denotes the offset
    /// of the opening byte
    /// of this AST node from some stream source.
    pub o: usize,

    /// line denotes the opening line
    /// of this AST node from some stream source.
    pub l: usize,

    /// ns denotes child nodes.
    pub ns: Vec<Gem>,
}

impl Mk {
    /// new constructs a makefile AST.
    pub fn new(ns: Vec<Gem>) -> Mk {
        Mk { o: 0, l: 1, ns }
    }
}

impl Default for Mk {
    /// default generates a basic makefile AST.
    fn default() -> Self {
        Mk::new(Vec::new())
    }
}

impl Traceable for Mk {
    /// set_offset applies the given offset.
    fn set_offset(&mut self, offset: usize) {
        self.o = offset;
    }

    /// get_offset queries the current offset.
    fn get_offset(&self) -> usize {
        self.o
    }

    /// set_line applies the given line.
    fn set_line(&mut self, line: usize) {
        self.l = line;
    }

    /// get_line queries the current line.
    fn get_line(&self) -> usize {
        self.l
    }

    /// update corrects line details.
    fn update(&mut self, index: &HashMap<Range<usize>, usize>) {
        for n in &mut self.ns {
            n.update(index);
        }
    }
}

parser! {
    grammar parser() for str {
        /// eof matches the end of a file.
        rule eof() = quiet!{![_]} / expected!("EOF")

        rule line_ending() -> String =
            quiet!{
                s:$"\n" {
                    s.to_string()
                }
            } / expected!("LF")

        rule macro_escaped_newline() -> String =
            quiet!{
                ("\\" line_ending() _) {
                    " ".to_string()
                }
            } / expected!("escaped LF")

        /// _ matches optional whitespace, with optional escaped newlines.
        ///
        /// Leading whitespace on successive lines is elided.
        rule _ = quiet!{(" " / "\t" / macro_escaped_newline())*} / expected!("whitespace")

        /// __ matches required whitespace, without allowing escaped newlines.
        rule __ = quiet!{(" " / "\t")+} / expected!("whitespace")

        rule escaped_non_line_ending() -> String =
            quiet!{
                s:$("\\" [^ (' ' | '\t' | '\r' | '\n')]) {
                    s.to_string()
                }
            } / expected!("c-style escape")

        rule comment() -> String =
            quiet!{
                ("#" ([^ ('\r' | '\n')]*) (line_ending() / eof())) {
                    String::new()
                }
            } / expected!("comment")

        rule macro_expansion() -> String =
            quiet!{
                s:$(("$(" macro_name() ")") / ("${" macro_name() "}")) {
                    s.to_string()
                }
            } / expected!("macro expansion")

        rule non_special_target_literal() -> String =
            quiet!{
                s:$([^ (' ' | '\t' | ':' | ';' | '=' | '#' | '\r' | '\n' | '\\')]+) {?
                    if SPECIAL_TARGETS.contains(s) {
                        Err("special target")
                    } else {
                        Ok(s.to_string())
                    }
                }
            } / expected!("target")

        rule target() -> String =
            s:$(non_special_target_literal() / macro_expansion()) {
                s.to_string()
            }

        rule wait_prerequisite() -> String =
            quiet!{
                s:$(".WAIT") {
                    s.to_string()
                }
            } / expected!("wait prerequisite marker")

        rule prerequisite() -> String =
            s:$(non_special_target_literal() / wait_prerequisite() / macro_expansion()) {
                s.to_string()
            }

        rule simple_command() -> String =
            quiet!{
                s:$([^ ('\r' | '\n' | '\\')]+ escaped_non_line_ending()* [^ ('\r' | '\n' | '\\')]*) {
                    s.to_string()
                }
            } / expected!("command")

        rule command_escaped_newline() -> String =
            s:$("\\" line_ending()) "\t"*<0,1> {
                s.to_string()
            }

        rule multiline_command() -> String =
            a:command_escaped_newline() b:compound_make_command() {
                format!("{}{}", a, b)
            }

        rule compound_make_command() -> String =
            s:(simple_command() / multiline_command()) {
                s.to_string()
            }

        rule make_command() -> String =
            strings:(compound_make_command()+) {
                strings.join("")
            }

        rule inline_command() -> String =
            quiet!{
                ";" _ strings:make_command()*<0,1> {
                    strings.join("")
                }
            } / expected!("inline command")

        rule indented_command() -> String =
            (comment() / line_ending())* "\t" s:make_command() (line_ending()+ / eof()) {
                s.to_string()
            }

        rule with_prerequisites() -> (Vec<String>, Vec<String>) =
            ps:(prerequisite() ++ _) _ inline_commands:(inline_command()*<0, 1>) ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()*) {
                (ps, [inline_commands, indented_commands].concat())
            }

        rule with_prerequisites_without_commands() -> (Vec<String>, Vec<String>) =
            ps:(prerequisite() ++ _) _ ((comment() / line_ending())+ / eof()) {
                (ps, Vec::new())
            }

        rule commands_with_inline() -> Vec<String> =
            inline_commands:(inline_command()*<1,1>) ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()*) {
                [inline_commands, indented_commands].concat()
            }

        rule commands_without_inline() -> Vec<String> =
            ((comment() / line_ending())+) indented_commands:(indented_command()+) {
                indented_commands
            }

        rule without_prerequisites() -> (Vec<String>, Vec<String>) =
            cs:(commands_with_inline() / commands_without_inline()) {
                (Vec::new(), cs)
            }

        rule without_prerequisites_without_commands() -> (Vec<String>, Vec<String>) =
            ((comment() / line_ending())+ / eof()) {
                (Vec::new(), Vec::new())
            }

        rule special_unit_target() -> String =
            quiet!{
                s:$("." ("POSIX" / "NOTPARALLEL" / "WAIT")) {
                    s.to_string()
                }
            } / expected!("target")

        rule special_unit_rule() -> (Vec<String>, (Vec<String>, Vec<String>)) =
            t:special_unit_target() _ ":" _ pcs:without_prerequisites_without_commands() {
                (vec![t.to_string()], pcs)
            }

        rule special_commands_target() -> String =
            quiet!{
                s:$("." ("DEFAULT" / "SCCS_GET")) {
                    s.to_string()
                }
            } / expected!("target")

        rule special_commands_rule() -> (Vec<String>, (Vec<String>, Vec<String>)) =
            t:special_commands_target() _ ":" _ pcs:without_prerequisites() {
                (vec![t.to_string()], pcs)
            }

        rule special_config_target() -> String =
            quiet!{
                s:$("." ("IGNORE" / "PHONY" / "PRECIOUS" / "SILENT" / "SUFFIXES")) {
                    s.to_string()
                }
            } / expected!("target")

        rule special_target_config_rule() -> (Vec<String>, (Vec<String>, Vec<String>)) =
            t:special_config_target() _ ":" _ pcs:(with_prerequisites_without_commands() / without_prerequisites_without_commands()) {
                (vec![t.to_string()], pcs)
            }

        rule special_target_rule() -> Gem =
            (comment() / line_ending())* p:position!() tpcs:(special_unit_rule() / special_commands_rule() / special_target_config_rule()) {
                let (ts, (ps, cs)) = tpcs;

                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Ru {
                        ts,
                        ps,
                        cs: cs.into_iter().filter(|e| !e.is_empty()).collect(),
                    }
                }
            }

        rule make_rule() -> Gem =
            (comment() / line_ending())* p:position!() ts:(target() ++ _) _ ":" _ pcs:(with_prerequisites() / without_prerequisites()) {
                let (ps, cs) = pcs;

                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Ru {
                        ts,
                        ps,
                        cs: cs.into_iter().filter(|e| !e.is_empty()).collect(),
                    },
                }
            }

        rule macro_name_literal() -> String =
            quiet!{
                s:$(['.' | '_' | '0'..='9' | 'a'..='z' | 'A'..='Z']+) {
                    s.to_string()
                }
            } / expected!("macro name literal")

        rule macro_name() -> String =
            comment()* s:$(macro_name_literal() / macro_expansion()) {
                s.to_string()
            }

        rule macro_value_literal() -> String =
            quiet!{
                s:$([^ ('\r' | '\n' | '\\' | '#')]+) {
                    s.to_string()
                }
            } / expected!("macro value literal")

        rule multiline_macro_value() -> String =
            a:macro_escaped_newline() b:compound_macro_value() {
                format!("{}{}", a, b)
            }

        rule compound_macro_value() -> String =
            s:(macro_value_literal() / multiline_macro_value() / escaped_non_line_ending()) {
                s.to_string()
            }

        rule macro_value() -> String =
            strings:(compound_macro_value()*) ((comment() / line_ending())+ / eof()) {
                strings.join("")
            }

        rule assignment_operator() -> String =
            quiet!{
                s:$(("+" / "!" / "?" / ":::" / "::")*<0,1> "=") {
                    s.to_string()
                }
            } / expected!("assignment operator")

        rule macro_definition() -> Gem =
            (comment() / line_ending())* p:position!() n:macro_name() _ assignment_operator() _ v:macro_value() {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Mc {
                        n,
                        v,
                    },
                }
            }

        rule include_value_literal() -> String =
            quiet!{
                s:$([^ ('"' | ' ' | '\r' | '\n' | '\\' | '#')]+) {
                    s.to_string()
                }
            } / expected!("include value literal")

        rule include_value() -> String =
            s:$(include_value_literal() / macro_expansion()) {
                s.to_string()
            }

        rule include_opening() -> String =
            quiet!{
                s:$("-include" / "include") {
                    s.to_string()
                }
            } / expected!("include opening")

        rule include() -> Gem =
            (comment() / line_ending())* p:position!() include_opening() __ ps:(include_value() ++ _) _ ((comment() / line_ending())+ / eof()) {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::In {
                        ps,
                    },
                }
            }

        rule general_expression() -> Gem =
            (comment() / line_ending())* p:position!() expression:macro_expansion() args:(macro_value()?) {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Ex {
                        e: format!("{}{}", expression, args.unwrap_or(String::new())),
                    },
                }
            }

        rule node() -> Gem =
            n:(special_target_rule() / make_rule() / include() / macro_definition() / general_expression()) {
                n
            }

        pub rule parse() -> Mk =
            (comment() / line_ending())* ns:(node()*) (comment() / line_ending())* {
                Mk::new(ns)
            }
    }
}

/// parse_posix generates a makefile AST from a string.
pub fn parse_posix(pth: &str, s: &str) -> Result<Mk, String> {
    let mut ast: Mk = parser::parse(s).map_err(|err| {
        let loc: peg::str::LineCol = err.location;

        let mut valid_tokens: Vec<&str> = err
            .expected
            .tokens()
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect();
        valid_tokens.sort();

        let bad_token: String = s
            .chars()
            .nth(loc.offset)
            .map(|e| format!("\"{}\"", e.to_string().escape_debug()))
            .unwrap_or("EOF".to_string());

        format!(
            "error: {}:{}:{} found {}, expected: {}",
            pth,
            loc.line,
            loc.column,
            bad_token,
            valid_tokens.join(", ")
        )
    })?;

    let index: HashMap<Range<usize>, usize> = [
        vec![0],
        s.match_indices('\n').map(|(offset, _)| offset).collect(),
        vec![s.len()],
    ]
    .concat()
    .windows(2)
    .enumerate()
    .map(|(i, window)| {
        (
            Range {
                start: window[0],
                end: window[1],
            },
            1 + i,
        )
    })
    .collect();

    ast.update(&index);
    Ok(ast)
}

#[test]
fn test_grammar() {
    use self::walkdir;
    use std::fs;
    use std::path;

    let fixtures_path: &path::Path = path::Path::new("fixtures");
    let valid_walker = walkdir::WalkDir::new(fixtures_path.join("parse-valid")).sort_by_file_name();

    for entry_result in valid_walker {
        let entry: walkdir::DirEntry = entry_result.unwrap();
        let pth: &path::Path = entry.path();

        if pth.is_dir() {
            continue;
        }

        let pth_string: String = pth.display().to_string();
        let makefile_str: &str = &fs::read_to_string(&pth).unwrap();
        assert!(parse_posix(&pth_string, makefile_str)
            .map_err(|err| format!("unable to parse {}: {}", &pth_string, err))
            .is_ok());
    }

    let invalid_walker = walkdir::WalkDir::new(fixtures_path.join("parse-invalid"))
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !e.path().is_dir());

    for entry_result in invalid_walker {
        let entry: walkdir::DirEntry = entry_result.unwrap();
        let pth: &path::Path = entry.path();

        if pth.is_dir() {
            continue;
        }

        let pth_string: String = pth.display().to_string();
        let makefile_str: &str = &fs::read_to_string(&pth).unwrap();
        assert!(
            parse_posix(&pth_string, makefile_str).is_err(),
            "failed to reject {}",
            &pth_string
        );
    }
}

#[test]
fn test_whitespace() {
    assert_eq!(
        parse_posix("-", "\n\ninclude  \tfoo.mk bar.mk \t\tbaz.mk \t\n\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            ps: vec![
                "foo.mk".to_string(),
                "bar.mk".to_string(),
                "baz.mk".to_string(),
            ]
        }]
    );

    assert_eq!(
        parse_posix("-", "BLANK  =  \n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "BLANK".to_string(),
            v: String::new(),
        }]
    );

    assert_eq!(
        parse_posix("-", "\n\nC  \t=  c \n\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "C".to_string(),
            v: "c ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("-", "\n\na-2.txt\tb-2.txt \tc-2.txt \t: \ta-1.txt\tb-1.txt \tc-1.txt \t\n\n\tcp a-1.txt a-2.txt\n\tcp b-1.txt b-2.txt\n\tcp c-1.txt c-2.txt \t\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ps: vec![
                "a-1.txt".to_string(),
                "b-1.txt".to_string(),
                "c-1.txt".to_string(),
            ],
            ts: vec![
                "a-2.txt".to_string(),
                "b-2.txt".to_string(),
                "c-2.txt".to_string(),
            ],
            cs: vec![
                "cp a-1.txt a-2.txt".to_string(),
                "cp b-1.txt b-2.txt".to_string(),
                "cp c-1.txt c-2.txt \t".to_string(),
            ],
        }]
    );

    assert!(parse_posix("-", " \n").is_err());

    assert_eq!(
        parse_posix("-", "include abc\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            ps: vec!["abc".to_string()]
        }]
    );

    assert!(parse_posix("-", "includeabc\n").is_err());
}

#[test]
fn test_comments() {
    assert_eq!(
        parse_posix(
            "-",
            "\n# place foo.mk contents here\ninclude foo.mk\n# End of file\n"
        )
        .unwrap()
        .ns
        .into_iter()
        .map(|e| e.n)
        .collect::<Vec<Ore>>(),
        vec![Ore::In {
            ps: vec!["foo.mk".to_string()]
        }]
    );

    assert_eq!(
        parse_posix("-", "\n# C references a character\nC=c\n# End of file\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "C".to_string(),
            v: "c".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("-", "\n# foo is an application binary\nfoo:foo.c\n\n# gcc is a common Linux compiler\n\tgcc -o foo foo.c\n# End of file\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ps: vec!["foo.c".to_string()],
            ts: vec!["foo".to_string()],
            cs: vec!["gcc -o foo foo.c".to_string()],
        }]
    );
}

#[test]
fn test_offsets_and_line_numbers() {
    assert_eq!(
        parse_posix("-", "# alphabet\nA=apple").unwrap().ns,
        vec![Gem {
            o: 11,
            l: 2,
            n: Ore::Mc {
                n: "A".to_string(),
                v: "apple".to_string(),
            }
        }]
    );
}

#[test]
fn test_c_family_escape_preservation() {
    assert_eq!(
        parse_posix("-", "all:\n\tprintf \"Hello World!\\\n\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["printf \"Hello World!\\\n\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("-", "MSG=\"Hello World!\\n\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "MSG".to_string(),
            v: "\"Hello World!\\n\"".to_string(),
        }]
    );
}

#[test]
fn test_multiline_expressions() {
    assert_eq!(
        parse_posix("-", "FULL_NAME\\\n=\\\n \tAlice\\\n \tLiddell\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "FULL_NAME".to_string(),
            v: "Alice Liddell".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("-", "foo: foo.c\n\tgcc\\\n-o foo\\\n\tfoo.c\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ps: vec!["foo.c".to_string()],
            ts: vec!["foo".to_string()],
            cs: vec!["gcc\\\n-o foo\\\nfoo.c".to_string()],
        }]
    );

    assert_eq!(
        parse_posix(
            "-",
            "report-1 \\\nreport-2 \\\n \treport-3\\\n:\\\ntest-1\\\ntest-2\\\n \ttest-3\n"
        )
        .unwrap()
        .ns
        .into_iter()
        .map(|e| e.n)
        .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ps: vec![
                "test-1".to_string(),
                "test-2".to_string(),
                "test-3".to_string(),
            ],
            ts: vec![
                "report-1".to_string(),
                "report-2".to_string(),
                "report-3".to_string(),
            ],
            cs: Vec::new(),
        }]
    );
}
