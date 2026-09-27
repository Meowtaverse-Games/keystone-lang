use std::collections::HashSet;

use keystone_lang::{Error, validate_allowed_commands};

fn allowed(commands: &[&str]) -> HashSet<String> {
    commands
        .iter()
        .map(|command| (*command).to_string())
        .collect()
}

#[test]
fn rejects_disallowed_commands_inside_unexecuted_bodies() {
    let cases = [
        ("if false\n    dig down\nend", "dig"),
        ("loop 0\n    sleep 1.0\nend", "sleep"),
        ("while false\n    move right\nend", "move"),
        ("if false\n    print is_touched()\nend", "is_touched"),
        ("if false\n    print is_empty(left)\nend", "is_empty"),
    ];

    for (source, command) in cases {
        assert_eq!(
            validate_allowed_commands(source, &allowed(&[])),
            Err(Error::CommandNotAllowed {
                command: command.to_string(),
            }),
            "{source}",
        );
    }
}

#[test]
fn leaves_general_language_constructs_available() {
    let source = r#"
        label = "dig"
        count = rand(3)
        send label
        receive "stone-1"
        turn left
        print count
    "#;

    assert_eq!(validate_allowed_commands(source, &allowed(&[])), Ok(()));
}

#[test]
fn accepts_only_the_commands_explicitly_allowed() {
    let source = r#"
        loop 1
            if is_touched() and is_empty(right)
                move right
                sleep 1.0
                dig down
            end
        end
    "#;

    assert_eq!(
        validate_allowed_commands(
            source,
            &allowed(&["move", "sleep", "dig", "is_touched", "is_empty"]),
        ),
        Ok(())
    );
}
