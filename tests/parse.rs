use std::{collections::HashSet, sync::Arc};

use keystone_lang::{
    Direction, Error, Expr, ExternalApi, Statement, eval, eval_all, parse,
    validate_allowed_commands,
};

#[test]
fn parse_returns_statements_without_type_checking() {
    // `x` is undefined, which the type checker would reject, but parsing alone succeeds.
    let statements = parse("move forward\nprint x").unwrap();

    assert_eq!(
        statements,
        vec![
            Statement::Move(Expr::Direction(Direction::Forward)),
            Statement::Print(Expr::Var("x".to_string())),
        ]
    );
}

#[test]
fn parse_accepts_empty_input() {
    assert_eq!(parse("").unwrap(), Vec::new());
    assert_eq!(parse("  \n").unwrap(), Vec::new());
}

#[test]
fn parse_reports_syntax_errors() {
    assert!(matches!(
        parse("move forward\nloop"),
        Err(Error::SyntaxError { .. })
    ));
}

#[test]
fn parse_accepts_maximum_u32_literal() {
    assert_eq!(
        parse("move 4294967295").unwrap(),
        vec![Statement::Move(Expr::Uint(u32::MAX))]
    );
}

#[test]
fn parse_still_accepts_floats_above_u32_max() {
    assert_eq!(
        parse("x = 4294967296.0").unwrap(),
        vec![Statement::Let("x".to_string(), Expr::Float(4294967296.0))]
    );
}

fn assert_oversized_literal_error(result: Result<(), Error>, input: &str, literal: &str) {
    let Err(Error::SyntaxError { messages }) = result else {
        panic!("expected a syntax error for {input:?}, got {result:?}");
    };
    let start = input.find(literal).unwrap();
    let span = format!("{}..{}", start, start + literal.len());
    assert!(
        messages.iter().any(|message| {
            message.contains("number literal is too large (max 4294967295)")
                && message.contains(&span)
        }),
        "expected an oversized literal error at {span}, got {messages:?}"
    );
}

struct Api;

impl ExternalApi for Api {
    fn is_touched(&self) -> bool {
        panic!("invalid syntax must not reach the external API")
    }

    fn is_empty(&self, _: Direction) -> bool {
        panic!("invalid syntax must not reach the external API")
    }

    fn send_signal(&self, _: &str) {
        panic!("invalid syntax must not reach the external API")
    }

    fn receive_signal(&self, _: &str) -> bool {
        panic!("invalid syntax must not reach the external API")
    }
}

#[test]
fn public_parsing_entry_points_report_oversized_literals() {
    for (input, literal) in [
        ("move 4294967296", "4294967296"),
        ("move 99999999999", "99999999999"),
        ("x = 1 + 99999999999", "99999999999"),
        ("loop 99999999999\nmove forward\nend", "99999999999"),
        ("x = rand(99999999999)", "99999999999"),
        ("move forward\nx = 99999999999", "99999999999"),
    ] {
        assert_oversized_literal_error(parse(input).map(|_| ()), input, literal);
        assert_oversized_literal_error(eval(input, Arc::new(Api)).map(|_| ()), input, literal);
        assert_oversized_literal_error(eval_all(input, Arc::new(Api)).map(|_| ()), input, literal);
        assert_oversized_literal_error(
            validate_allowed_commands(input, &HashSet::new()),
            input,
            literal,
        );
    }
}
