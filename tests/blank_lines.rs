use std::sync::Arc;

use keystone_lang::{Direction, Error, Event, ExternalApi, eval_all, parse};

struct Api;

impl ExternalApi for Api {
    fn is_touched(&self) -> bool {
        false
    }

    fn is_empty(&self, _: Direction) -> bool {
        false
    }

    fn send_signal(&self, _: &str) {}

    fn receive_signal(&self, _: &str) -> bool {
        false
    }
}

fn run(source: &str) -> Result<Vec<Event>, Error> {
    eval_all(source, Arc::new(Api))
}

fn prints(source: &str) -> Vec<String> {
    run(source)
        .unwrap()
        .into_iter()
        .filter_map(|event| match event {
            Event::Print(text) => Some(text),
            _ => None,
        })
        .collect()
}

/// Blank lines are only layout, so they must not change what the program means.
fn assert_same_as_without_blank_lines(source: &str) {
    let compact: String = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    assert_eq!(
        parse(source),
        parse(&compact),
        "blank lines changed the meaning of {source:?}"
    );
    assert!(parse(source).is_ok(), "{source:?} should parse");
}

#[test]
fn blank_lines_inside_every_kind_of_body_are_accepted() {
    for source in [
        "loop 2\n    print 1\n\n    print 2\nend",
        "while false\n    print 1\n\n    print 2\nend",
        "if true\n    print 1\n\n    print 2\nend",
        "if false\n    print 1\nelse\n    print 2\n\n    print 3\nend",
        "if false\n    print 1\nelse if true\n    print 2\n\n    print 3\nend",
    ] {
        assert_same_as_without_blank_lines(source);
    }
}

#[test]
fn blank_lines_at_the_start_and_end_of_a_body_are_accepted() {
    for source in [
        "loop 1\n\n    print 1\nend",
        "loop 1\n    print 1\n\nend",
        "if true\n\n    print 1\n\nelse\n\n    print 2\n\nend",
        "if false\n\nelse if true\n\n    print 1\n\nend",
    ] {
        assert_same_as_without_blank_lines(source);
    }
}

#[test]
fn several_blank_lines_in_a_row_are_accepted() {
    assert_same_as_without_blank_lines("loop 1\n    print 1\n\n\n\n    print 2\nend");
}

#[test]
fn a_body_of_only_blank_lines_is_empty() {
    for source in [
        "loop 3\n\nend",
        "while false\n\n\nend",
        "if true\n\nelse\n\nend",
    ] {
        assert_same_as_without_blank_lines(source);
    }
}

#[test]
fn lines_with_only_spaces_or_tabs_count_as_blank() {
    for source in [
        "loop 1\n    print 1\n    \n    print 2\nend",
        "if true\n    print 1\n\t\n    print 2\nend",
        "if false\nelse\n        \n    print 1\nend",
    ] {
        assert_same_as_without_blank_lines(source);
    }
}

#[test]
fn trailing_spaces_after_a_line_are_accepted() {
    // Typing a space after a condition used to be a syntax error.
    assert_eq!(prints("loop 2   \n    print 1   \nend"), ["1", "1"]);
    assert_eq!(prints("if true \n    print \"a\" \nend"), ["a"]);
    assert_eq!(prints("while false\t\n    print 1\nend\nprint 2"), ["2"]);
    assert_eq!(
        prints("if false \n    print 1\nelse if true \n    print 2\nend"),
        ["2"]
    );
}

#[test]
fn windows_line_endings_with_blank_lines_are_accepted() {
    assert_eq!(
        prints("loop 2\r\n    print 1\r\n\r\n    print 2\r\nend"),
        ["1", "2", "1", "2"]
    );
}

#[test]
fn nested_bodies_with_blank_lines_run_as_written() {
    let source = r#"
        count = 0

        loop 2

            if count == 0

                print "first"

            else

                print "again"

            end

            count = count + 1

        end

        print count
    "#;

    assert_eq!(prints(source), ["first", "again", "2"]);
    assert_same_as_without_blank_lines(source);
}

#[test]
fn a_missing_end_is_still_a_syntax_error() {
    for source in [
        "loop 2\n    print 1\n\n",
        "if true\n\n    print 1\n\nelse\n\n",
        "while false\n\n",
    ] {
        assert!(
            matches!(parse(source), Err(Error::SyntaxError { .. })),
            "{source:?} should be a syntax error"
        );
    }
}

#[test]
fn two_statements_on_one_line_inside_a_body_are_still_rejected() {
    assert!(matches!(
        parse("loop 1\n    print 1 print 2\nend"),
        Err(Error::SyntaxError { .. })
    ));
}
