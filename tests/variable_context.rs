use std::{panic, sync::Arc};

use keystone_lang::{Direction, Error, Event, ExternalApi, Type, eval_all};

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

fn name_error(name: &str) -> Result<Vec<Event>, Error> {
    Err(Error::NameError {
        name: name.to_string(),
    })
}

fn type_changed(name: &str, from: Type, to: Type) -> Result<Vec<Event>, Error> {
    Err(Error::TypeChanged {
        name: name.to_string(),
        from,
        to,
    })
}

#[test]
fn programs_that_used_to_panic_are_rejected_before_running() {
    for source in [
        "if false\n    y = 1\nend\nprint y",
        "if is_touched()\n    y = 1\nend\nprint y",
        "loop 0\n    y = 1\nend\nprint y",
        "while false\n    y = 1\nend\nprint y",
        "x = \"a\"\nif false\n    x = left\nend\nmove x",
        "x = \"a\"\nif false\n    x = 3\nend\nprint x",
        "if false\n    y = 1\nelse if false\n    y = 2\nend\nprint y",
    ] {
        let result = panic::catch_unwind(|| run(source));
        assert!(
            matches!(result, Ok(Err(_))),
            "{source:?} should be an error, got {result:?}"
        );
    }
}

#[test]
fn variables_from_a_body_are_not_visible_after_it() {
    assert_eq!(run("if true\n    y = 1\nend\nprint y"), name_error("y"));
    assert_eq!(run("loop 1\n    y = 1\nend\nprint y"), name_error("y"));
    assert_eq!(run("while false\n    y = 1\nend\nprint y"), name_error("y"));
    assert_eq!(
        run("if true\n    y = 1\nelse if true\n    y = 2\nend\nprint y"),
        name_error("y")
    );
}

#[test]
fn variables_are_visible_later_in_their_own_body() {
    let source = r#"
        loop 2
            y = "inner"
            if true
                z = y
                print z
            end
            print y
        end
    "#;

    assert_eq!(prints(source), ["inner", "inner", "inner", "inner"]);
}

#[test]
fn a_loop_body_cannot_read_a_variable_before_setting_it() {
    // The second iteration would see `y`, but the first one would not.
    assert_eq!(run("loop 2\n    print y\n    y = 1\nend"), name_error("y"));
}

#[test]
fn variables_set_before_a_body_can_be_updated_inside_it() {
    let source = r#"
        count = 0
        loop 3
            count = count + 1
        end
        if count == 3
            count = 10
        end
        print count
    "#;

    assert_eq!(prints(source), ["10"]);
}

#[test]
fn a_variable_set_in_every_branch_is_visible_after_an_if_else() {
    let source = r#"
        if false
            y = "a"
        else if false
            y = "b"
        else
            y = "c"
        end
        print y
    "#;

    assert_eq!(prints(source), ["c"]);
}

#[test]
fn a_name_can_be_reused_after_its_body_ends() {
    let source = r#"
        if true
            y = 1
        end
        y = "again"
        print y
    "#;

    assert_eq!(prints(source), ["again"]);
}

#[test]
fn a_variable_keeps_the_type_of_its_first_assignment() {
    assert_eq!(
        run("x = \"a\"\nx = 3"),
        type_changed("x", Type::String, Type::Uint)
    );
    assert_eq!(
        run("x = \"a\"\nif false\n    x = 3\nend"),
        type_changed("x", Type::String, Type::Uint)
    );
    assert_eq!(
        run("x = 1\nloop 1\n    x = left\nend"),
        type_changed("x", Type::Uint, Type::Direction)
    );
    assert_eq!(
        run("x = 1\nwhile false\n    x = true\nend"),
        type_changed("x", Type::Uint, Type::Boolean)
    );
    assert_eq!(
        run("x = 1\nif true\nelse\n    x = \"a\"\nend"),
        type_changed("x", Type::Uint, Type::String)
    );
}

#[test]
fn uint_and_float_are_different_types() {
    assert_eq!(
        run("speed = 1\nspeed = 0.5"),
        type_changed("speed", Type::Uint, Type::Float)
    );
}

#[test]
fn reassigning_the_same_type_is_allowed() {
    assert_eq!(prints("x = \"a\"\nx = \"b\"\nprint x"), ["b"]);
}

#[test]
fn branches_may_introduce_the_same_name_with_different_types() {
    // Each branch is checked on its own; the name is just not usable afterwards.
    let source = r#"
        if true
            x = "text"
            print x
        else
            x = 1
            print x
        end
    "#;

    assert_eq!(prints(source), ["text"]);
}
