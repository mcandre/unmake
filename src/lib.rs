//! unmake provides predicates for analyzing makefiles.

extern crate lazy_static;
extern crate peg;

use peg::parser;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;

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
        rule _ = (" " / "\t")*

        rule eof() = ![_]

        rule line_ending() -> String =
            s:$("\n") {
                s.to_string()
            }

        rule escaped_non_line_ending() -> String =
            s:$("\\" [^ ('\r' | '\n')]) {
                s.to_string()
            }

        rule comment() -> String =
            (("#" / "-include") ([^ ('\r' | '\n')]*) (line_ending() / eof())) {
                String::new()
            }

        rule simple_prerequisite() -> String =
            s:$([^ (' ' | '\t' | ':' | ';' | '=' | '#' | '\r' | '\n' | '\\')]+) {
                s.to_string()
            }

        rule make_prerequisite() -> String =
            s:$(simple_prerequisite() / "$(" simple_prerequisite() ")" / "${" simple_prerequisite() "}") {
                s.to_string()
            }

        rule simple_command_value() -> String =
            s:$([^ ('\r' | '\n' | '\\')]+) {
                s.to_string()
            }

        rule command_escaped_newline() -> String =
            s:$("\\" line_ending()) "\t"*<0,1> {
                s.to_string()
            }

        rule multiline_command() -> String =
            a:command_escaped_newline() b:compound_make_command() {
                format!("{}{}", a, b)
            }

        rule compound_make_command() -> String =
            s:(simple_command_value() / multiline_command() / escaped_non_line_ending()) {
                s.to_string()
            }

        rule make_command() -> String =
            strings:(compound_make_command()+) {
                strings.join("")
            }

        rule inline_command() -> String =
            ";" _ strings:make_command()*<0,1> {
                strings.join("")
            }

        rule indented_command() -> String =
            (comment() / line_ending())* "\t" s:make_command() (line_ending()+ / eof()) {
                s.to_string()
            }

        rule with_prerequisites() -> (Vec<String>, Vec<String>) =
            ps:(make_prerequisite() ++ _) _ inline_commands:(inline_command()*<0, 1>) ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()*) {
                (ps, [inline_commands, indented_commands].concat())
            }

        rule commands_with_inline() -> Vec<String> =
            inline_commands:(inline_command()*<1,1>) ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()*) {
                [inline_commands, indented_commands].concat()
            }

        rule commands_without_inline() -> Vec<String> =
            ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()+) {
                indented_commands
            }

        rule without_prerequisites() -> (Vec<String>, Vec<String>) =
            cs:(commands_with_inline() / commands_without_inline()) {
                (Vec::new(), cs)
            }

        rule make_rule() -> Gem =
            (comment() / line_ending())* p:position!() ts:(make_prerequisite() ++ _) _ ":" _ pcs:(with_prerequisites() / without_prerequisites()) {
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

        rule simple_macro_name() -> String =
            s:$(['.' | '_' | '0'..='9' | 'a'..='z' | 'A'..='Z']+) {
                s.to_string()
            }

        rule macro_name() -> String =
            comment()* s:$(simple_macro_name() / "$(" simple_macro_name() ")" / "${" simple_macro_name() "}") {
                s.to_string()
            }

        rule macro_escaped_newline() -> String =
            ("\\" line_ending()) {
                " ".to_string()
            }

        rule simple_macro_value() -> String =
            s:$([^ ('\r' | '\n' | '\\' | '#')]+) {
                s.to_string()
            }

        rule multiline_macro_value() -> String =
            a:macro_escaped_newline() b:compound_macro_value() {
                format!("{}{}", a, b)
            }

        rule compound_macro_value() -> String =
            s:(simple_macro_value() / multiline_macro_value() / escaped_non_line_ending()) {
                s.to_string()
            }

        rule macro_value() -> String =
            strings:(compound_macro_value()*) ((comment() / line_ending())+ / eof()) {
                strings.join("")
            }

        rule macro_definition() -> Gem =
            (comment() / line_ending())* p:position!() n:macro_name() _ ("+=" / "!=" / "?=" / ":::=" / "::=" / "=") _ v:macro_value() {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Mc {
                        n,
                        v,
                    },
                }
            }

        rule include_value() -> String =
            s:$([^ ('"' | ' ' | '\r' | '\n' | '\\' | '#')]+) {
                s.to_string()
            }

        rule include() -> Gem =
            (comment() / line_ending())* p:position!() "include" _ ps:(include_value() ++ _) _ ((comment() / line_ending())+ / eof()) {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::In {
                        ps,
                    },
                }
            }

        rule general_expression() -> Gem =
            (comment() / line_ending())* p:position!() command:$("$(" _ simple_macro_name() _ ")" / "${" _ simple_macro_name() _ "}") args:(macro_value()?) {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Ex {
                        e: format!("{}{}", command, args.unwrap_or(String::new())),
                    },
                }
            }

        rule node() -> Gem =
            n:(macro_definition() / make_rule() / include() / general_expression()) {
                n
            }

        pub rule parse() -> Mk =
            (comment() / line_ending())* ns:(node()*) (comment() / line_ending())* {
                Mk::new(ns)
            }
    }
}

/// parse_posix generates a makefile AST from a string.
pub fn parse_posix(s: &str) -> Result<Mk, String> {
    let mut ast: Mk = parser::parse(s).map_err(|err| err.to_string())?;
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
    use std::fs;
    use std::path;

    let fixtures_path: &path::Path = path::Path::new("fixtures");
    let valid_fixture_paths: Vec<path::PathBuf> = fs::read_dir(fixtures_path.join("valid"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    for pth in valid_fixture_paths {
        let makefile_str: &str = &fs::read_to_string(&pth).unwrap();
        assert_eq!(
            parse_posix(makefile_str).map(|_| ()).map_err(|err| format!(
                "unable to parse {}: {}",
                pth.display(),
                err
            )),
            Ok(())
        );
    }

    let invalid_fixture_paths: Vec<path::PathBuf> = fs::read_dir(fixtures_path.join("invalid"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    for pth in invalid_fixture_paths {
        let makefile_str: &str = &fs::read_to_string(&pth).unwrap();
        assert!(
            parse_posix(makefile_str).is_err(),
            "failed to reject {}",
            pth.display()
        );
    }
}

#[test]
fn test_whitespace() {
    assert_eq!(
        parse_posix("\n\ninclude  \tfoo.mk bar.mk \t\tbaz.mk \t\n\n")
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
        parse_posix("BLANK  =  \n")
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
        parse_posix("\n\nC  \t=  c \n\n")
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
        parse_posix("\n\na-2.txt\tb-2.txt \tc-2.txt \t: \ta-1.txt\tb-1.txt \tc-1.txt \t\n\n\tcp a-1.txt a-2.txt\n\tcp b-1.txt b-2.txt\n\tcp c-1.txt c-2.txt \t\n")
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
}

#[test]
fn test_comments() {
    assert_eq!(
        parse_posix("\n# place foo.mk contents here\ninclude foo.mk\n# End of file\n")
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
        parse_posix("\n# C references a character\nC=c\n# End of file\n")
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
        parse_posix("\n# foo is an application binary\nfoo:foo.c\n\n# gcc is a common Linux compiler\n\tgcc -o foo foo.c\n# End of file\n")
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
        parse_posix("# alphabet\nA=apple").unwrap().ns,
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
fn test_multiline_expressions() {
    assert_eq!(
        parse_posix("FULL_NAME=Alice\\\nLiddell\n")
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
        parse_posix("foo: foo.c\n\tgcc\\\n-o foo\\\n\tfoo.c\n")
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
}
