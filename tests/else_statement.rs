use std::{collections::HashSet, sync::Arc};

use keystone_lang::{
    Direction, Error, Event, Expr, ExternalApi, Statement, eval_all, parse,
    validate_allowed_commands,
};

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

fn allowed(commands: &[&str]) -> HashSet<String> {
    commands
        .iter()
        .map(|command| (*command).to_string())
        .collect()
}

fn print(text: &str) -> Statement {
    Statement::Print(Expr::String(text.to_string()))
}

#[test]
fn parses_else_as_the_second_branch() {
    assert_eq!(
        parse("if true\n    print \"a\"\nelse\n    print \"b\"\nend").unwrap(),
        vec![Statement::If(
            Expr::Boolean(true),
            vec![print("a")],
            Some(vec![print("b")]),
        )]
    );
}

#[test]
fn if_without_else_has_no_else_branch() {
    assert_eq!(
        parse("if true\n    print \"a\"\nend").unwrap(),
        vec![Statement::If(Expr::Boolean(true), vec![print("a")], None)]
    );
}

#[test]
fn empty_else_is_kept_apart_from_a_missing_else() {
    assert_eq!(
        parse("if true\nelse\nend").unwrap(),
        vec![Statement::If(Expr::Boolean(true), vec![], Some(vec![]))]
    );
}

#[test]
fn else_if_chain_nests_in_the_else_branch_and_shares_one_end() {
    let source = r#"
        if false
            print "a"
        else if true
            print "b"
        else
            print "c"
        end
    "#;

    assert_eq!(
        parse(source).unwrap(),
        vec![Statement::If(
            Expr::Boolean(false),
            vec![print("a")],
            Some(vec![Statement::If(
                Expr::Boolean(true),
                vec![print("b")],
                Some(vec![print("c")]),
            )]),
        )]
    );
}

#[test]
fn runs_exactly_the_first_matching_branch() {
    let source = |n: u32| {
        format!(
            r#"
            n = {n}
            if n == 1
                print "one"
            else if n == 2
                print "two"
            else if n < 5
                print "few"
            else
                print "many"
            end
            print "after"
            "#
        )
    };

    assert_eq!(prints(&source(1)), ["one", "after"]);
    assert_eq!(prints(&source(2)), ["two", "after"]);
    assert_eq!(prints(&source(3)), ["few", "after"]);
    assert_eq!(prints(&source(9)), ["many", "after"]);
}

#[test]
fn else_if_without_a_final_else_can_run_nothing() {
    let source = r#"
        if false
            print "a"
        else if false
            print "b"
        end
        print "after"
    "#;

    assert_eq!(prints(source), ["after"]);
}

#[test]
fn later_conditions_are_not_evaluated_after_a_match() {
    let source = r#"
        if true
            print "a"
        else if 1 / 0 == 0
            print "b"
        end
    "#;

    assert_eq!(prints(source), ["a"]);
}

#[test]
fn branches_nest_inside_loops_and_each_other() {
    let source = r#"
        i = 0
        loop 3
            i = i + 1
            if i == 1
                if false
                    print "x"
                else
                    print "first"
                end
            else if i == 2
                print "second"
            else
                print "third"
            end
        end
    "#;

    assert_eq!(prints(source), ["first", "second", "third"]);
}

#[test]
fn extra_spaces_around_else_if_are_accepted() {
    assert_eq!(
        prints("if false\nelse   if true\n    print \"a\"\nelse   \nend"),
        ["a"]
    );
}

#[test]
fn identifiers_may_start_with_else() {
    let source = r#"
        else_count = 1
        if true
            elsewhere = 2
            print elsewhere
        else
            print else_count
        end
    "#;

    assert_eq!(prints(source), ["2"]);
}

#[test]
fn rejects_misplaced_else() {
    for source in [
        "else\nend",
        "print \"a\"\nelse\nend",
        "if true\nelse\nelse\nend",
        "if true\nelse\nelse if true\nend",
        "if true\nelseif true\nend",
        "if true\nelse if\nend",
        "if true\nelse",
        "loop 1\nelse\nend",
        "while false\nelse\nend",
    ] {
        assert!(
            matches!(parse(source), Err(Error::SyntaxError { .. })),
            "{source:?} should be a syntax error"
        );
    }
}

#[test]
fn else_is_reserved() {
    assert!(matches!(
        parse("print else"),
        Err(Error::SyntaxError { .. })
    ));
}

#[test]
fn else_if_condition_must_be_boolean() {
    assert!(matches!(
        run("if false\nelse if 1\nend"),
        Err(Error::UnexpectedType { .. })
    ));
}

#[test]
fn branch_bodies_are_type_checked() {
    for source in [
        "if true\nelse\n    move 1\nend",
        "if true\nelse if true\n    move 1\nend",
    ] {
        assert!(
            matches!(run(source), Err(Error::UnexpectedType { .. })),
            "{source:?} should be rejected"
        );
    }
}

#[test]
fn else_cannot_read_a_variable_set_in_the_then_branch() {
    assert!(run("if true\n    x = 1\nelse\n    print x\nend").is_err());
}

#[test]
fn variable_set_in_both_branches_is_available_afterwards() {
    let source = r#"
        if false
            x = "then"
        else
            x = "else"
        end
        print x
    "#;

    assert_eq!(prints(source), ["else"]);
}

#[test]
fn variable_set_in_one_branch_is_rejected_afterwards() {
    for source in [
        "if false\n    x = 1\nelse\nend\nprint x",
        "if true\nelse\n    x = 1\nend\nprint x",
    ] {
        assert!(run(source).is_err(), "{source:?} should be rejected");
    }
}

#[test]
fn variable_with_conflicting_branch_types_is_rejected_afterwards() {
    let source = r#"
        if true
            x = "text"
        else
            x = left
        end
        move x
    "#;

    assert!(run(source).is_err());
}

#[test]
fn variable_from_before_the_if_stays_available() {
    let source = r#"
        x = "before"
        if true
        else
        end
        print x
    "#;

    assert_eq!(prints(source), ["before"]);
}

#[test]
fn capabilities_cover_else_branches_and_else_if_conditions() {
    for (source, command) in [
        ("if true\nelse\n    dig up\nend", "dig"),
        ("if true\nelse if true\n    place up\nend", "place"),
        ("if true\nelse if is_touched()\nend", "is_touched"),
        ("if true\nelse if false\nelse\n    sleep 1\nend", "sleep"),
    ] {
        assert!(
            matches!(
                validate_allowed_commands(source, &allowed(&["move"])),
                Err(Error::CommandNotAllowed { command: actual }) if actual == command
            ),
            "{source:?} should reject {command}"
        );
    }
}
