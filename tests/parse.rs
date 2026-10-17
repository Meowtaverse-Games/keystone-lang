use keystone_lang::{Direction, Error, Expr, Statement, parse};

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
