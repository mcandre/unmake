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
#[derive(Debug, PartialEq)]
pub enum Ore {
    Ru {
        ts: Vec<String>,
        ps: Vec<String>,
        cs: Vec<String>,
    },
    Mc {
        n: String,
        v: String,
    },
    In {
        p: String,
    },
    Ex {
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
            ("#" ([^ ('\r' | '\n')]*) (line_ending() / eof())) {
                String::new()
            }

        rule simple_prerequisite() -> String =
            s:$([^ (' ' | '\t' | ':' | ';' | '#' | '\r' | '\n' | '\\')]+) {
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
            ps:(make_prerequisite() ++ _) inline_commands:(inline_command()*<0, 1>) ((comment() / line_ending())+ / eof()) indented_commands:(indented_command()*) {
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
            (comment() / line_ending())* p:position!() ts:(make_prerequisite() ++ " ") _ ":" _ pcs:(with_prerequisites() / without_prerequisites()) {
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
            (comment() / line_ending())* p:position!() n:macro_name() _ "=" _ v:macro_value() {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::Mc {
                        n,
                        v,
                    },
                }
            }

        rule simple_path() -> String =
            s:$([^ ('"' | '\r' | '\n' | '\\' | '#')]+) {
                s.to_string()
            }

        rule include_value() -> String =
            s:simple_path() ((comment() / line_ending())+ / eof()) {
                s.trim_end().to_string()
            }

        rule include() -> Gem =
            (comment() / line_ending())* p:position!() "include" _ s:include_value() {
                Gem {
                    o: p,
                    l: 0,
                    n: Ore::In {
                        p: s.to_string(),
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
            n:(make_rule() / include() / macro_definition() / general_expression()) {
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
fn test_isolated_comment_lines() {
    assert_eq!(
        parse_posix(""),
        Ok(Mk {
            o: 0,
            l: 1,
            ns: Vec::new()
        })
    );
    assert_eq!(
        parse_posix("\n"),
        Ok(Mk {
            o: 0,
            l: 1,
            ns: Vec::new()
        })
    );

    assert!(parse_posix("\r").is_err());
    assert!(parse_posix("\r\n").is_err());
    assert!(parse_posix("\r\n\r\n").is_err());

    assert_eq!(parse_posix("\n\n").unwrap().ns, Vec::new());
    assert_eq!(parse_posix("#\n").unwrap().ns, Vec::new());
    assert_eq!(parse_posix("#").unwrap().ns, Vec::new());
    assert_eq!(parse_posix("# alphabet\n").unwrap().ns, Vec::new());
    assert_eq!(parse_posix("# alphabet").unwrap().ns, Vec::new());

    assert_eq!(
        parse_posix("# alphabet\n# a, b, c, ... z\n").unwrap().ns,
        Vec::new(),
    );
    assert_eq!(
        parse_posix("# alphabet\n# a, b, c, ... z").unwrap().ns,
        Vec::new(),
    );
}

#[test]
fn test_parse_macros() {
    assert_eq!(
        parse_posix("A=1\n").unwrap().ns,
        vec![Gem {
            o: 0,
            l: 1,
            n: Ore::Mc {
                n: "A".to_string(),
                v: "1".to_string(),
            },
        }]
    );

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

    assert_eq!(
        parse_posix("A=1\nB=2\n"),
        Ok(Mk::new(vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::Mc {
                    n: "A".to_string(),
                    v: "1".to_string(),
                },
            },
            Gem {
                o: 4,
                l: 2,
                n: Ore::Mc {
                    n: "B".to_string(),
                    v: "2".to_string(),
                },
            }
        ]))
    );

    assert_eq!(
        parse_posix("A=1\n\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=1\n\n\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=1")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A =1")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A  =1")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A = 1")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=1 ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1 ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A= 1 ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1 ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A = 1 ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1 ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A  =1  ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1  ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A\t=1")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=1 \n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1 ".to_string(),
        }]
    );

    assert!(parse_posix("A=1\r\n").is_err());

    assert_eq!(
        parse_posix("A=1\n\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "1".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=\"Alice\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "\"Alice\"".to_string()
        }]
    );

    assert_eq!(
        parse_posix("A='Alice'")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "'Alice'".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: String::new(),
        }]
    );

    assert_eq!(
        parse_posix("A=\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: String::new(),
        }]
    );

    assert_eq!(
        parse_posix("A= ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A= \n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("BLANK=\n")
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
        parse_posix("A=apple# alphabet")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=apple # alphabet")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "apple ".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=apple\n# alphabet")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=apple\n# alphabet\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=1\nB=2")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "1".to_string(),
            },
            Ore::Mc {
                n: "B".to_string(),
                v: "2".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("A=1\n\nB=2\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "1".to_string(),
            },
            Ore::Mc {
                n: "B".to_string(),
                v: "2".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("A=1\n\n\nB=2\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "1".to_string(),
            },
            Ore::Mc {
                n: "B".to_string(),
                v: "2".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("A=x\\\ny")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "x y".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=x\\\n y")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "x  y".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=x\\\n  y")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "x   y".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=x\\\n\ty")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "x \ty".to_string(),
        }]
    );

    assert!(parse_posix("A=x\\ \ny").is_err());

    assert_eq!(
        parse_posix("A=x\\\ny\nB=z")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "x y".to_string(),
            },
            Ore::Mc {
                n: "B".to_string(),
                v: "z".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("B=Hello\\\nWorld!")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "B".to_string(),
            v: "Hello World!".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("C=")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "C".to_string(),
            v: "".to_string(),
        }]
    );

    assert!(parse_posix("A").is_err());
    assert!(parse_posix("=1").is_err());

    assert_eq!(
        parse_posix("A==apple")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "=apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A= =apple")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "=apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A = =apple")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "A".to_string(),
            v: "=apple".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("A=B\n$(B)=C\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "B".to_string(),
            },
            Ore::Mc {
                n: "$(B)".to_string(),
                v: "C".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("A=B\n$(B)=C")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "B".to_string(),
            },
            Ore::Mc {
                n: "$(B)".to_string(),
                v: "C".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("A=B\n${B}=C\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "A".to_string(),
                v: "B".to_string(),
            },
            Ore::Mc {
                n: "${B}".to_string(),
                v: "C".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("LF=\"\\n\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "LF".to_string(),
            v: "\"\\n\"".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("INTERNALS=$@ $% $? $< $^ $*\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Mc {
            n: "INTERNALS".to_string(),
            v: "$@ $% $? $< $^ $*".to_string(),
        }]
    );

    assert!(parse_posix("A=\\\n").is_err());
    assert!(parse_posix("A=\\").is_err());
}

#[test]
fn test_parse_general_expressions() {
    assert_eq!(
        parse_posix("I=include\n$(I) a.mk\n").unwrap().ns,
        vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::Mc {
                    n: "I".to_string(),
                    v: "include".to_string(),
                },
            },
            Gem {
                o: 10,
                l: 2,
                n: Ore::Ex {
                    e: "$(I) a.mk".to_string(),
                },
            },
        ]
    );

    assert_eq!(
        parse_posix("I=include\nM=a.mk\n$(I)\\\n$(M)").unwrap().ns,
        vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::Mc {
                    n: "I".to_string(),
                    v: "include".to_string(),
                },
            },
            Gem {
                o: 10,
                l: 2,
                n: Ore::Mc {
                    n: "M".to_string(),
                    v: "a.mk".to_string(),
                },
            },
            Gem {
                o: 17,
                l: 3,
                n: Ore::Ex {
                    e: "$(I) $(M)".to_string(),
                },
            },
        ]
    );

    assert_eq!(
        parse_posix("I=include\n$(I) a.mk")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "I".to_string(),
                v: "include".to_string(),
            },
            Ore::Ex {
                e: "$(I) a.mk".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("I=include\n\n\n$(I) a.mk\n\n\n$(I) b.mk\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "I".to_string(),
                v: "include".to_string(),
            },
            Ore::Ex {
                e: "$(I) a.mk".to_string(),
            },
            Ore::Ex {
                e: "$(I) b.mk".to_string(),
            },
        ]
    );

    assert_eq!(
        parse_posix("I=include\nM=a.mk\n$(I) $(M)")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "I".to_string(),
                v: "include".to_string(),
            },
            Ore::Mc {
                n: "M".to_string(),
                v: "a.mk".to_string(),
            },
            Ore::Ex {
                e: "$(I) $(M)".to_string(),
            },
        ]
    );
}

#[test]
fn test_parse_includes() {
    assert_eq!(
        parse_posix("include a.mk\n").unwrap().ns,
        vec![Gem {
            o: 0,
            l: 1,
            n: Ore::In {
                p: "a.mk".to_string(),
            },
        },]
    );

    assert_eq!(
        parse_posix("include a.mk\n\n\ninclude b.mk").unwrap().ns,
        vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::In {
                    p: "a.mk".to_string(),
                },
            },
            Gem {
                o: 15,
                l: 4,
                n: Ore::In {
                    p: "b.mk".to_string(),
                },
            },
        ]
    );

    assert_eq!(
        parse_posix("\ninclude a.mk\n").unwrap().ns,
        vec![Gem {
            o: 1,
            l: 2,
            n: Ore::In {
                p: "a.mk".to_string(),
            },
        },]
    );

    assert_eq!(
        parse_posix("include a.mk")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert!(parse_posix("include \"a.mk\"\n").is_err());

    assert_eq!(
        parse_posix("include\ta.mk")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("include  a.mk")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("include a.mk ")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("include a.mk# task definitions")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("include a.mk # task definitions")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::In {
            p: "a.mk".to_string(),
        }]
    );

    assert_eq!(
        parse_posix("PTH=a.mk\ninclude $(PTH)")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "PTH".to_string(),
                v: "a.mk".to_string(),
            },
            Ore::In {
                p: "$(PTH)".to_string(),
            },
        ]
    );

    assert!(parse_posix("include a\\\n.mk").is_err());
}

#[test]
fn test_rules() {
    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n").unwrap().ns,
        vec![Gem {
            o: 0,
            l: 1,
            n: Ore::Ru {
                ts: vec!["all".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\"".to_string()],
            },
        }]
    );

    assert_eq!(
        parse_posix("all: ;echo \"Hello World!\"\n").unwrap().ns,
        vec![Gem {
            o: 0,
            l: 1,
            n: Ore::Ru {
                ts: vec!["all".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\"".to_string()],
            },
        }]
    );

    assert_eq!(
        parse_posix("all: ;echo \"Hello World!\"\n\techo \"Hi!\"\n")
            .unwrap()
            .ns,
        vec![Gem {
            o: 0,
            l: 1,
            n: Ore::Ru {
                ts: vec!["all".to_string()],
                ps: Vec::new(),
                cs: vec![
                    "echo \"Hello World!\"".to_string(),
                    "echo \"Hi!\"".to_string(),
                ],
            },
        }]
    );

    assert!(parse_posix("all:\n").is_err());
    assert!(parse_posix("all:; echo \"Hello World!\"\n; echo \"Hi!\"\n").is_err());

    assert_eq!(
        parse_posix("all:\n\n\n# emit console message\n\n\n\techo \"Hello World!\"\n# emitted hello world\n\techo \"Hi World!\"\n").unwrap().ns,
        vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::Ru{
                    ts: vec!["all".to_string()],
                    ps: Vec::new(),
                    cs: vec![
                        "echo \"Hello World!\"".to_string(),
                        "echo \"Hi World!\"".to_string(),
                    ],
                },
            },
        ]
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .ns,
        vec![
            Gem {
                o: 0,
                l: 1,
                n: Ore::Ru {
                    ts: vec!["test-1".to_string()],
                    ps: Vec::new(),
                    cs: vec!["echo \"Hello World!\"".to_string()],
                },
            },
            Gem {
                o: 29,
                l: 3,
                n: Ore::Ru {
                    ts: vec!["test-2".to_string()],
                    ps: Vec::new(),
                    cs: vec!["echo \"Hi World!\"".to_string()],
                },
            },
        ]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"# emit console message\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"# emit console message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\" # emit console message\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\" # emit console message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\t# echo \"Hello World!\"\n\techo \"Hi World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec![
                "# echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        }]
    );

    assert!(parse_posix("all:\n        echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n       echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n      echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n     echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n    echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n   echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n  echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\n echo \"Hello World!\"\n").is_err());
    assert!(parse_posix("all:\necho \"Hello World!\"\n").is_err());

    assert_eq!(
        parse_posix("all:\n\techo \\\n\t\"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \\\n\"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \\\n\"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\t\t\"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \\\n\t\"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\t\t\"Hello World!\"\\\n\t\t\"Hi World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \\\n\t\"Hello World!\"\\\n\t\"Hi World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\"\n\n\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru {
                ts: vec!["test-1".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\"".to_string()],
            },
            Ore::Ru {
                ts: vec!["test-2".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hi World!\"".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("# some tests\ntest-1:\n\techo \"Hello World!\"\n# even more tests\ntest-2:\n\techo \"Hi World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru{
                ts: vec!["test-1".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\"".to_string()],
            },
            Ore::Ru{
                ts: vec!["test-2".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hi World!\"".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\" # some tests\ntest-2:\n\techo \"Hi World!\" # even more tests\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru{
                ts: vec!["test-1".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\" # some tests".to_string()],
            },
            Ore::Ru{
                ts: vec!["test-2".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hi World!\" # even more tests".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n# End\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n# End")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("# default task\nall:\n\techo \"Hello World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:# emit a console message\n\techo \"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all: # emit a console message\n\techo \"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n# emit a console message\n\techo \"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"# emit a console message\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"# emit a console message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\" # emit a console message\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\" # emit a console message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\" # emit a console message")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\" # emit a console message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all: test\ntest: hello\n\t./hello\nhello: hello.c\n\tcc -o hello hello.c")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru {
                ts: vec!["all".to_string()],
                ps: vec!["test".to_string()],
                cs: Vec::new(),
            },
            Ore::Ru {
                ts: vec!["test".to_string()],
                ps: vec!["hello".to_string()],
                cs: vec!["./hello".to_string()],
            },
            Ore::Ru {
                ts: vec!["hello".to_string()],
                ps: vec!["hello.c".to_string()],
                cs: vec!["cc -o hello hello.c".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix(
            "test: test-hi test-howdy\n\ntest-hi:\n\techo hi\n\ntest-howdy:\n\techo howdy\n"
        )
        .unwrap()
        .ns
        .into_iter()
        .map(|e| e.n)
        .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru {
                ts: vec!["test".to_string()],
                ps: vec!["test-hi".to_string(), "test-howdy".to_string()],
                cs: Vec::new(),
            },
            Ore::Ru {
                ts: vec!["test-hi".to_string()],
                ps: Vec::new(),
                cs: vec!["echo hi".to_string()],
            },
            Ore::Ru {
                ts: vec!["test-howdy".to_string()],
                ps: Vec::new(),
                cs: vec!["echo howdy".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("coverage.html coverage.xml:\n\tcover")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["coverage.html".to_string(), "coverage.xml".to_string()],
            ps: Vec::new(),
            cs: vec!["cover".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n# End\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n# End")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("# integration test\ntest:; echo \"Hello World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\" # emit message\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\" # emit message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\" # emit message")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Hello World!\" # emit message".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n\techo \"Hi World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec![
                "echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        }]
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n\techo \"Hi World!\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec![
                "echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        }]
    );

    assert_eq!(
        parse_posix("test1:; echo \"Hello World!\"\ntest2:\n\techo \"Hi World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Ru {
                ts: vec!["test1".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hello World!\"".to_string()],
            },
            Ore::Ru {
                ts: vec!["test2".to_string()],
                ps: Vec::new(),
                cs: vec!["echo \"Hi World!\"".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("rule: ;\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["rule".to_string()],
            ps: Vec::new(),
            cs: Vec::new(),
        }]
    );

    assert_eq!(
        parse_posix("rule: ;")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["rule".to_string()],
            ps: Vec::new(),
            cs: Vec::new(),
        }]
    );

    assert_eq!(
        parse_posix("test:\n\techo \"Hello World!\"\n\n\techo \"Hi World!\"")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["test".to_string()],
            ps: Vec::new(),
            cs: vec![
                "echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Welcome:\\n - Alice\\n - Bob\\n - Carol\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"Welcome:\\n - Alice\\n - Bob\\n - Carol\"".to_string()],
        }]
    );

    assert!(parse_posix("all:\\\n").is_err());
    assert!(parse_posix("all:\\").is_err());

    assert_eq!(
        parse_posix("BIN=hello\n$(BIN): hello.c\n\tcc -o $(BIN) hello.c\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "BIN".to_string(),
                v: "hello".to_string(),
            },
            Ore::Ru {
                ts: vec!["$(BIN)".to_string()],
                ps: vec!["hello.c".to_string()],
                cs: vec!["cc -o $(BIN) hello.c".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("BIN=hello\n$(BIN): hello.c\n\tcc -o $(BIN) hello.c")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "BIN".to_string(),
                v: "hello".to_string(),
            },
            Ore::Ru {
                ts: vec!["$(BIN)".to_string()],
                ps: vec!["hello.c".to_string()],
                cs: vec!["cc -o $(BIN) hello.c".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("BIN=hello\n${BIN}: hello.c\n\tcc -o ${BIN} hello.c\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "BIN".to_string(),
                v: "hello".to_string(),
            },
            Ore::Ru {
                ts: vec!["${BIN}".to_string()],
                ps: vec!["hello.c".to_string()],
                cs: vec!["cc -o ${BIN} hello.c".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("BANNER=hello-v0.0.1\n$(BANNER).zip:\n\tzip -r $(BANNER).zip $(BANNER)\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "BANNER".to_string(),
                v: "hello-v0.0.1".to_string(),
            },
            Ore::Ru {
                ts: vec!["$(BANNER).zip".to_string()],
                ps: Vec::new(),
                cs: vec!["zip -r $(BANNER).zip $(BANNER)".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("BANNER=hello-v0.0.1\n$(BANNER).zip:\n\tzip -r $(BANNER).zip $(BANNER)")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![
            Ore::Mc {
                n: "BANNER".to_string(),
                v: "hello-v0.0.1".to_string(),
            },
            Ore::Ru {
                ts: vec!["$(BANNER).zip".to_string()],
                ps: Vec::new(),
                cs: vec!["zip -r $(BANNER).zip $(BANNER)".to_string()],
            },
        ]
    );

    assert_eq!(
        parse_posix("all:\n\techo $@ $% $? $< $^ $*\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo $@ $% $? $< $^ $*".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("all:\n\techo \"PWD: $$PWD\"\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["all".to_string()],
            ps: Vec::new(),
            cs: vec!["echo \"PWD: $$PWD\"".to_string()],
        }]
    );

    assert_eq!(
        parse_posix("lib: lib(file1.o) lib(file2.o) lib(file3.o)\n\t@echo lib is now up-to-date\n")
            .unwrap()
            .ns
            .into_iter()
            .map(|e| e.n)
            .collect::<Vec<Ore>>(),
        vec![Ore::Ru {
            ts: vec!["lib".to_string()],
            ps: vec![
                "lib(file1.o)".to_string(),
                "lib(file2.o)".to_string(),
                "lib(file3.o)".to_string(),
            ],
            cs: vec!["@echo lib is now up-to-date".to_string()]
        }]
    );
}
