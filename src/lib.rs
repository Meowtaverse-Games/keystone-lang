use nom::{
    IResult,
    bytes::complete::{tag},
    character::complete::{digit1, char, multispace0, alpha1},
    sequence::{preceded},
    combinator::{map, all_consuming},
    branch::alt,
    multi::many1,
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expr),
    Print(Expr),
    If(Box<Expr>, Vec<Statement>),
    Loop(i64, Vec<Statement>),
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |s: &str| Expr::Number(s.parse::<i64>().unwrap()))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(alpha1, |s: &str| Expr::Variable(s.to_string()))(input)
}

fn parse_operator(input: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-"), tag("*"), tag("/")))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (input, left) = alt((parse_number, parse_variable))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, op) = parse_operator(input)?;
    let (input, _) = multispace0(input)?;
    let (input, right) = alt((parse_number, parse_variable))(input)?;

    Ok((input, Expr::BinaryOp(Box::new(left), op.to_string(), Box::new(right))))
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, var) = parse_variable(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_expression(input)?;

    Ok((input, Statement::Assignment(
        match var {
            Expr::Variable(s) => s,
            _ => panic!("Invalid variable"),
        },
        value,
    )))
}

fn parse_print(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("print")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expression(input)?;

    Ok((input, Statement::Print(expr)))
}

fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, cond) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("then")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, statements) = many1(parse_statement)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("end")(input)?;

    Ok((input, Statement::If(Box::new(cond), statements)))
}

fn parse_loop(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("loop")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, n) = digit1(input)?;
    let n: i64 = n.parse().unwrap();
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("times")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, statements) = many1(parse_statement)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("end")(input)?;

    Ok((input, Statement::Loop(n, statements)))
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_assignment, parse_print, parse_if, parse_loop))(input)
}

fn parse_statements(input: &str) -> IResult<&str, Vec<Statement>> {
    many1(preceded(multispace0, parse_statement))(input)
}

fn evaluate_expr(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(v) => *vars.get(v).unwrap_or(&0),
        Expr::BinaryOp(left, op, right) => {
            let left_val = evaluate_expr(left, vars);
            let right_val = evaluate_expr(right, vars);
            match op.as_str() {
                "+" => left_val + right_val,
                "-" => left_val - right_val,
                "*" => left_val * right_val,
                "/" => left_val / right_val,
                _ => 0,
            }
        }
    }
}

fn execute_statement(statement: &Statement, vars: &mut HashMap<String, i64>) {
    match statement {
        Statement::Assignment(var_name, expr) => {
            let value = evaluate_expr(expr, &vars);
            vars.insert(var_name.clone(), value);
        }
        Statement::Print(expr) => {
            let value = evaluate_expr(expr, &vars);
            println!("{}", value);
        }
        Statement::If(cond, body) => {
            let cond_value = evaluate_expr(cond, &vars);
            if cond_value != 0 {
                for stmt in body {
                    execute_statement(stmt, vars);
                }
            }
        }
        Statement::Loop(times, body) => {
            for _ in 0..*times {
                for stmt in body {
                    execute_statement(stmt, vars);
                }
            }
        }
    }
}

fn execute_code(code: &str) {
    let mut vars: HashMap<String, i64> = HashMap::new();

    let (_, statements) = all_consuming(parse_statements)(code)
        .expect("Failed to parse full input");

    for stmt in &statements {
        execute_statement(stmt, &mut vars);
    }
}

pub fn run(code: &str) {
    execute_code(code);
}
