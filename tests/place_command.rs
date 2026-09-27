use std::{collections::HashSet, sync::Arc};

use keystone_lang::{
    Direction, Error, Event, ExternalApi, Type, eval_all, validate_allowed_commands,
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

fn allowed(commands: &[&str]) -> HashSet<String> {
    commands
        .iter()
        .map(|command| (*command).to_string())
        .collect()
}

#[test]
fn emits_place_events_for_all_product_directions() {
    let events = eval_all(
        "place up\nplace down\nplace left\nplace right",
        Arc::new(Api),
    )
    .expect("place commands should evaluate");

    assert_eq!(
        events,
        vec![
            Event::Place(Direction::Up),
            Event::Place(Direction::Down),
            Event::Place(Direction::Left),
            Event::Place(Direction::Right),
        ]
    );
}

#[test]
fn requires_a_direction() {
    assert_eq!(
        eval_all("place 1", Arc::new(Api)),
        Err(Error::UnexpectedType {
            statement: "Place".to_string(),
            found_type: Type::Uint,
        })
    );
}

#[test]
fn validates_place_in_unexecuted_branches() {
    assert_eq!(
        validate_allowed_commands("if false\n    place up\nend", &allowed(&[])),
        Err(Error::CommandNotAllowed {
            command: "place".to_string(),
        })
    );
    assert_eq!(
        validate_allowed_commands("if false\n    place up\nend", &allowed(&["place"])),
        Ok(())
    );
}
