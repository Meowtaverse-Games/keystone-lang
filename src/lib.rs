use chumsky::prelude::*;

#[derive(Debug,Clone)]
enum Statement {
    Print(Value),
}

#[derive(Debug,Clone)]
enum Value {
    Number(u32),
    String(String),
    Boolean(bool),
}

fn value_parser<'a>() -> impl Parser<'a, &'a str, Value, extra::Err<Simple<'a, char>>> {
    just("true").to(Value::Boolean(true))
}


fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Simple<'a, char>>> {
    just("print")
        .padded()
        .ignore_then(value_parser())
        .map(Statement::Print)
}

fn program_parser<'a>() -> impl Parser<'a, &'a str, Vec<Statement>, extra::Err<Simple<'a, char>>> {
    statement_parser()
        .separated_by(text::newline())
        .collect()
        .then_ignore(end())
}

pub fn analyze() {
    let parser = program_parser();

    let input = r#"print true"#;
    let output = parser.parse(input);
    println!("{:?}", output);
}