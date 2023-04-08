//! unmake provides predicates for analyzing makefiles.

extern crate lazy_static;
extern crate peg;

use peg::parser;

/// Directive models assorted Makefile token types.
/// Comments may be elided during persing.
#[derive(Debug, PartialEq)]
pub enum Directive {
    /// Rule denotes a rule token with a sequence of target names, a sequence of prerequisites, and a sequence of commands.
    Rule(Vec<String>, Vec<String>, Vec<String>),

    /// Macro denotes a macro token with a name and an expression value.
    Macro(String, String),

    /// Include denotes an include token.
    Include(String),

    /// GeneralExp denotes a general expression,
    /// which is expected to expand to another directive type.
    GeneralExp(String),
}

/// Makefile models a makefile AST.
#[derive(Debug, PartialEq)]
pub struct Makefile {
    /// directives denotes assorted Makefile data types.
    pub directives: Vec<Directive>,
}

impl Makefile {
    /// new constructs a Makefile AST.
    pub fn new(directives: Vec<Directive>) -> Makefile {
        Makefile { directives }
    }
}

impl Default for Makefile {
    /// default generates a basic Makefile AST.
    fn default() -> Self {
        Makefile::new(Vec::new())
    }
}

parser! {
    grammar parser() for str {
        rule _ = (" " / "\t")*

        rule line_ending() -> String =
            s:$("\r\n" / "\n") {
                s.to_string()
            }

        rule simple_value() -> String =
            s:$([^ ('\r' | '\n' | '\\' | '#')]+) {
                s.to_string()
            }

        rule macro_escaped_newline() -> String =
            ("\\" line_ending()) {
                " ".to_string()
            }

        rule comment() -> String =
            ("#" [^ ('\r' | '\n')]*) {
                String::new()
            }

        rule simple_macro_name() -> String =
            s:$(['.' | '_' | '0'..='9' | 'a'..='z' | 'A'..='Z']+) {
                s.to_string()
            }

        rule macro_name() -> String =
            comment()* s:$(simple_macro_name() / "$(" simple_macro_name() ")" / "${" simple_macro_name() "}") {
                s.to_string()
            }

        rule line_ending_or_eof() -> String =
            (line_ending() / ![_]) {
                String::new()
            }

        rule macro_value() -> String =
            strings:((simple_value() / macro_escaped_newline())*) (comment() / line_ending()+ / ![_]) {
                strings.join("")
            }

        rule simple_path() -> String =
            s:$([^ ('"' | '\r' | '\n' | '\\' | '#')]+) {
                s.to_string()
            }

        rule include_value() -> String =
            s:simple_path() (comment() / line_ending()+ / ![_]) {
                s.trim_end().to_string()
            }

        rule include() -> Directive =
            "include" _ s:include_value() {
                Directive::Include(s.to_string())
            }

        rule macro_definition() -> Directive =
            n:macro_name() _ "=" v:macro_value() {
                Directive::Macro(n, v)
            }

        rule simple_prerequisite() -> String =
            s:$([^ (' ' | '\t' | ':' | ';' | '#' | '\r' | '\n')]+) {
                s.to_string()
            }

        rule make_prerequisite() -> String =
            s:$(simple_prerequisite() / "$(" simple_prerequisite() ")" / "${" simple_prerequisite() "}") {
                s.to_string()
            }

        rule command_escaped_newline() -> String =
            s:$("\\" line_ending()) "\t"*<0,1> {
                s.to_string()
            }

        rule make_command() -> String =
            strings:((simple_value() / command_escaped_newline())*) {
                strings.join("")
            }

        rule indented_command() -> String =
            (comment() / line_ending())* "\t" s:make_command() (comment() / line_ending_or_eof()) {
                s.to_string()
            }

        rule inline_command() -> String =
            ";" _ s:make_command() {
                s.to_string()
            }

        rule make_rule() -> Directive =
            targets:(make_prerequisite() ++ _) _ ":" _ prerequisites:(make_prerequisite() ** _) inline_commands:(inline_command()*<0, 1>) (comment() / line_ending_or_eof()) indented_commands:(indented_command()*) ((comment() / line_ending())* / ![_]) {
                Directive::Rule(targets, prerequisites, [inline_commands, indented_commands].concat())
            }

        rule general_expression() -> Directive =
            command:$("$(" _ simple_macro_name() _ ")" / "${" _ simple_macro_name() _ "}") args:(macro_value()?) {
                Directive::GeneralExp(format!("{}{}", command, args.unwrap_or(String::new())))
            }

        /// parse generates a Makefile AST from a valid POSIX makefile content string,
        /// or else returns a parse error.
        pub rule parse() -> Makefile =
            (comment() / line_ending())* v:(make_rule() / include() / macro_definition() / general_expression())* (comment() / line_ending())* {
                Makefile{ directives: v }
            }
    }
}

/// parse_posix generates a Makefile AST from a string.
pub fn parse_posix(s: &str) -> Result<Makefile, peg::error::ParseError<peg::str::LineCol>> {
    parser::parse(s)
}

#[test]
fn test_comments() {
    assert_eq!(parse_posix(""), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("\r\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("\n\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("\r\n\r\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("#\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("#"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("# alphabet\n"), Ok(Makefile::new(Vec::new())));
    assert_eq!(parse_posix("# alphabet"), Ok(Makefile::new(Vec::new())));
    assert_eq!(
        parse_posix("# alphabet\n# a, b, c, ... z\n"),
        Ok(Makefile::new(Vec::new()))
    );
    assert_eq!(
        parse_posix("# alphabet\n# a, b, c, ... z"),
        Ok(Makefile::new(Vec::new()))
    );
}

#[test]
fn test_parse_macros() {
    assert_eq!(
        parse_posix("A=1\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\n\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\n\n\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A =1"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A  =1"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A = 1"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            " 1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1 "),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1 ".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A  =1  "),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1  ".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A\t=1"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1 \n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1 ".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\r\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\n\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "1".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=\"Alice\""),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "\"Alice\"".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A='Alice'"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "'Alice'".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A="),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            String::new(),
        )]))
    );

    assert_eq!(
        parse_posix("A=\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            String::new(),
        )]))
    );

    assert_eq!(
        parse_posix("A= "),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            " ".to_string(),
        )]))
    );

    assert_eq!(
        parse_posix("A= \n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            " ".to_string(),
        )]))
    );

    assert_eq!(
        parse_posix("BLANK=\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "BLANK".to_string(),
            String::new()
        )]))
    );

    assert_eq!(
        parse_posix("A=apple# alphabet"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=apple # alphabet"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "apple ".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("# alphabet\nA=apple"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=apple\n# alphabet"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=apple\n# alphabet\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=1\nB=2\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "1".to_string()),
            Directive::Macro("B".to_string(), "2".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=1\nB=2"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "1".to_string()),
            Directive::Macro("B".to_string(), "2".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=1\n\nB=2\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "1".to_string()),
            Directive::Macro("B".to_string(), "2".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=1\n\n\nB=2\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "1".to_string()),
            Directive::Macro("B".to_string(), "2".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=x\\\n"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "x ".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=x\\\ny"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "x y".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=x\\\n y"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "x  y".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=x\\\n  y"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "x   y".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=x\\\n\ty"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "x \ty".to_string()
        )]))
    );

    assert!(parse_posix("A=x\\ \ny").is_err());

    assert_eq!(
        parse_posix("A=x\\\ny\nB=z"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "x y".to_string()),
            Directive::Macro("B".to_string(), "z".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("B=Hello\\\nWorld!"),
        Ok(Makefile::new(vec![Directive::Macro(
            "B".to_string(),
            "Hello World!".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("C="),
        Ok(Makefile::new(vec![Directive::Macro(
            "C".to_string(),
            "".to_string()
        )]))
    );

    assert!(parse_posix("A").is_err());
    assert!(parse_posix("=1").is_err());

    assert_eq!(
        parse_posix("A==apple"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            "=apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A= =apple"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            " =apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A = =apple"),
        Ok(Makefile::new(vec![Directive::Macro(
            "A".to_string(),
            " =apple".to_string()
        )]))
    );

    assert_eq!(
        parse_posix("A=B\n$(B)=C\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "B".to_string()),
            Directive::Macro("$(B)".to_string(), "C".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=B\n$(B)=C"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "B".to_string()),
            Directive::Macro("$(B)".to_string(), "C".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("A=B\n${B}=C\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("A".to_string(), "B".to_string()),
            Directive::Macro("${B}".to_string(), "C".to_string()),
        ]))
    );
}

#[test]
fn test_parse_general_expressions() {
    assert_eq!(
        parse_posix("I=include\n$(I) a.mk\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("I".to_string(), "include".to_string()),
            Directive::GeneralExp("$(I) a.mk".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("I=include\n$(I) a.mk"),
        Ok(Makefile::new(vec![
            Directive::Macro("I".to_string(), "include".to_string()),
            Directive::GeneralExp("$(I) a.mk".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("I=include\n\n\n$(I) a.mk\n\n\n$(I) b.mk\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("I".to_string(), "include".to_string()),
            Directive::GeneralExp("$(I) a.mk".to_string()),
            Directive::GeneralExp("$(I) b.mk".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("I=include\nM=a.mk\n$(I) $(M)"),
        Ok(Makefile::new(vec![
            Directive::Macro("I".to_string(), "include".to_string()),
            Directive::Macro("M".to_string(), "a.mk".to_string()),
            Directive::GeneralExp("$(I) $(M)".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("I=include\nM=a.mk\n$(I)\\\n$(M)"),
        Ok(Makefile::new(vec![
            Directive::Macro("I".to_string(), "include".to_string()),
            Directive::Macro("M".to_string(), "a.mk".to_string()),
            Directive::GeneralExp("$(I) $(M)".to_string()),
        ]))
    );
}

#[test]
fn test_parse_includes() {
    assert_eq!(
        parse_posix("include a.mk\n"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("include a.mk"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert!(parse_posix("include \"a.mk\"\n").is_err());

    assert_eq!(
        parse_posix("include\ta.mk"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("include  a.mk"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("include a.mk "),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("include a.mk\n\n\ninclude b.mk"),
        Ok(Makefile::new(vec![
            Directive::Include("a.mk".to_string()),
            Directive::Include("b.mk".to_string()),
        ]))
    );

    assert_eq!(
        parse_posix("include a.mk# task definitions"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("include a.mk # task definitions"),
        Ok(Makefile::new(vec![Directive::Include("a.mk".to_string())]))
    );

    assert_eq!(
        parse_posix("PTH=a.mk\ninclude $(PTH)"),
        Ok(Makefile::new(vec![
            Directive::Macro("PTH".to_string(), "a.mk".to_string()),
            Directive::Include("$(PTH)".to_string()),
        ]))
    );

    assert!(parse_posix("include a\\\n.mk").is_err());
}

#[test]
fn test_rules() {
    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\""),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
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
        parse_posix("all:\n\techo \\\n\t\"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \\\n\"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \\\n\"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\t\t\"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \\\n\t\"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \\\n\t\t\"Hello World!\"\\\n\t\t\"Hi World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \\\n\t\"Hello World!\"\\\n\t\"Hi World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\"\ntest-2:\n\techo \"Hi World!\"\n"),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test-1".to_string()],
                Vec::new(),
                vec!["echo \"Hello World!\"".to_string()],
            ),
            Directive::Rule(
                vec!["test-2".to_string()],
                Vec::new(),
                vec!["echo \"Hi World!\"".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\"\n\n\ntest-2:\n\techo \"Hi World!\"\n"),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test-1".to_string()],
                Vec::new(),
                vec!["echo \"Hello World!\"".to_string()],
            ),
            Directive::Rule(
                vec!["test-2".to_string()],
                Vec::new(),
                vec!["echo \"Hi World!\"".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("# some tests\ntest-1:\n\techo \"Hello World!\"\n# even more tests\ntest-2:\n\techo \"Hi World!\"\n"),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test-1".to_string()],
                Vec::new(),
                vec!["echo \"Hello World!\"".to_string()],
            ),
            Directive::Rule(
                vec!["test-2".to_string()],
                Vec::new(),
                vec!["echo \"Hi World!\"".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("test-1:\n\techo \"Hello World!\" # some tests\ntest-2:\n\techo \"Hi World!\" # even more tests\n"),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test-1".to_string()],
                Vec::new(),
                vec!["echo \"Hello World!\" ".to_string()],
            ),
            Directive::Rule(
                vec!["test-2".to_string()],
                Vec::new(),
                vec!["echo \"Hi World!\" ".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n# End\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\"\n# End"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("# default task\nall:\n\techo \"Hello World!\""),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n# emit a console message\n\techo \"Hello World!\""),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\" # emit a console message\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\" ".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all:\n\techo \"Hello World!\" # emit a console message"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["all".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\" ".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("all: test\ntest: hello\n\t./hello\nhello: hello.c\n\tcc -o hello hello.c"),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["all".to_string()],
                vec!["test".to_string()],
                Vec::new()
            ),
            Directive::Rule(
                vec!["test".to_string()],
                vec!["hello".to_string()],
                vec!["./hello".to_string()],
            ),
            Directive::Rule(
                vec!["hello".to_string()],
                vec!["hello.c".to_string()],
                vec!["cc -o hello hello.c".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix(
            "test: test-hi test-howdy\n\ntest-hi:\n\techo hi\n\ntest-howdy:\n\techo howdy\n"
        ),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test".to_string()],
                vec!["test-hi".to_string(), "test-howdy".to_string()],
                Vec::new(),
            ),
            Directive::Rule(
                vec!["test-hi".to_string()],
                Vec::new(),
                vec!["echo hi".to_string()],
            ),
            Directive::Rule(
                vec!["test-howdy".to_string()],
                Vec::new(),
                vec!["echo howdy".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("coverage.html coverage.xml:\n\tcover"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["coverage.html".to_string(), "coverage.xml".to_string()],
            Vec::new(),
            vec!["cover".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\""),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n# End\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n# End"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("# integration test\ntest:; echo \"Hello World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\"".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\" # emit message\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\" ".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\" # emit message"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec!["echo \"Hello World!\" ".to_string()],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n\techo \"Hi World!\""),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec![
                "echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        )]))
    );

    assert_eq!(
        parse_posix("test:; echo \"Hello World!\"\n\techo \"Hi World!\"\n"),
        Ok(Makefile::new(vec![Directive::Rule(
            vec!["test".to_string()],
            Vec::new(),
            vec![
                "echo \"Hello World!\"".to_string(),
                "echo \"Hi World!\"".to_string(),
            ],
        )]))
    );

    assert_eq!(
        parse_posix("test1:; echo \"Hello World!\"\ntest2:\n\techo \"Hi World!\""),
        Ok(Makefile::new(vec![
            Directive::Rule(
                vec!["test1".to_string()],
                Vec::new(),
                vec!["echo \"Hello World!\"".to_string()],
            ),
            Directive::Rule(
                vec!["test2".to_string()],
                Vec::new(),
                vec!["echo \"Hi World!\"".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("BIN=hello\n$(BIN): hello.c\n\tcc -o $(BIN) hello.c\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("BIN".to_string(), "hello".to_string()),
            Directive::Rule(
                vec!["$(BIN)".to_string()],
                vec!["hello.c".to_string()],
                vec!["cc -o $(BIN) hello.c".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("BIN=hello\n$(BIN): hello.c\n\tcc -o $(BIN) hello.c"),
        Ok(Makefile::new(vec![
            Directive::Macro("BIN".to_string(), "hello".to_string()),
            Directive::Rule(
                vec!["$(BIN)".to_string()],
                vec!["hello.c".to_string()],
                vec!["cc -o $(BIN) hello.c".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("BIN=hello\n${BIN}: hello.c\n\tcc -o ${BIN} hello.c\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("BIN".to_string(), "hello".to_string()),
            Directive::Rule(
                vec!["${BIN}".to_string()],
                vec!["hello.c".to_string()],
                vec!["cc -o ${BIN} hello.c".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("BANNER=hello-v0.0.1\n$(BANNER).zip:\n\tzip -r $(BANNER).zip $(BANNER)\n"),
        Ok(Makefile::new(vec![
            Directive::Macro("BANNER".to_string(), "hello-v0.0.1".to_string()),
            Directive::Rule(
                vec!["$(BANNER).zip".to_string()],
                Vec::new(),
                vec!["zip -r $(BANNER).zip $(BANNER)".to_string()],
            ),
        ]))
    );

    assert_eq!(
        parse_posix("BANNER=hello-v0.0.1\n$(BANNER).zip:\n\tzip -r $(BANNER).zip $(BANNER)"),
        Ok(Makefile::new(vec![
            Directive::Macro("BANNER".to_string(), "hello-v0.0.1".to_string()),
            Directive::Rule(
                vec!["$(BANNER).zip".to_string()],
                Vec::new(),
                vec!["zip -r $(BANNER).zip $(BANNER)".to_string()],
            ),
        ]))
    );
}
