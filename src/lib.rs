mod api;
mod ast;
mod context;
mod error;
mod parser;
mod typecheck;
mod vm;

use std::collections::HashSet;
use std::sync::Arc;

use chumsky::prelude::*;
use context::RuntimeContext;
use parser::program_parser as parser;
use typecheck::check;
use vm::run;

pub use {
    api::ExternalApi,
    ast::{Callee, Direction, Expr, Op, Side, Statement, Type, UnaryOp},
    context::TypeContext,
    error::Error,
    typecheck::expr_check,
    vm::{Event, EventIterator},
};

fn parse_and_check(input: &str) -> Result<Vec<Statement>, Error> {
    let parsed = parser().parse(input.trim());
    let Some(parsed) = parsed.output() else {
        let mut msg: Vec<String> = Vec::new();
        for err in parsed.errors() {
            msg.push(err.to_string());
        }
        return Err(Error::SyntaxError { messages: msg });
    };
    let mut type_ctx = TypeContext::new();
    check(&parsed, &mut type_ctx)?;
    Ok(parsed.to_owned())
}

/// Rejects stone-specific commands and sensors that are absent from `allowed_commands`.
///
/// This validates parsed syntax, including commands in loop and conditional bodies. It does not
/// infer capabilities from identifiers, strings, comments, or runtime execution paths.
pub fn validate_allowed_commands(
    input: &str,
    allowed_commands: &HashSet<String>,
) -> Result<(), Error> {
    validate_statements(&parse_and_check(input)?, allowed_commands)
}

pub fn eval(input: &str, api: Arc<dyn ExternalApi + Send + Sync>) -> Result<EventIterator, Error> {
    let parsed = parse_and_check(input)?;
    let runtime_ctx = RuntimeContext::new();
    Ok(run(parsed, runtime_ctx, api))
}

pub fn eval_all(input: &str, api: Arc<dyn ExternalApi + Send + Sync>) -> Result<Vec<Event>, Error> {
    let events: Result<Vec<Event>, Error> = eval(input, api)?.collect();
    events
}

fn validate_statements(
    statements: &[Statement],
    allowed_commands: &HashSet<String>,
) -> Result<(), Error> {
    for statement in statements {
        match statement {
            Statement::Move(expression) => {
                require_command("move", allowed_commands)?;
                validate_expression(expression, allowed_commands)?;
            }
            Statement::Sleep(expression) => {
                require_command("sleep", allowed_commands)?;
                validate_expression(expression, allowed_commands)?;
            }
            Statement::Dig(expression) => {
                require_command("dig", allowed_commands)?;
                validate_expression(expression, allowed_commands)?;
            }
            Statement::Place(expression) => {
                require_command("place", allowed_commands)?;
                validate_expression(expression, allowed_commands)?;
            }
            Statement::Loop(expression, body) | Statement::While(expression, body) => {
                validate_expression(expression, allowed_commands)?;
                validate_statements(body, allowed_commands)?;
            }
            Statement::If(expression, body) => {
                validate_expression(expression, allowed_commands)?;
                validate_statements(body, allowed_commands)?;
            }
            Statement::Print(expression)
            | Statement::Turn(expression)
            | Statement::Let(_, expression)
            | Statement::Receive(expression)
            | Statement::Send(expression) => validate_expression(expression, allowed_commands)?,
        }
    }
    Ok(())
}

fn validate_expression(expression: &Expr, allowed_commands: &HashSet<String>) -> Result<(), Error> {
    match expression {
        Expr::Call { callee, args } => {
            match callee {
                Callee::IsTouched => require_command("is_touched", allowed_commands)?,
                Callee::IsEmpty => require_command("is_empty", allowed_commands)?,
                Callee::Rand => {}
            }
            for argument in args {
                validate_expression(argument, allowed_commands)?;
            }
        }
        Expr::Unary { exp, .. } => validate_expression(exp, allowed_commands)?,
        Expr::Binary { lhs, rhs, .. } => {
            validate_expression(lhs, allowed_commands)?;
            validate_expression(rhs, allowed_commands)?;
        }
        Expr::Uint(_)
        | Expr::Float(_)
        | Expr::String(_)
        | Expr::Boolean(_)
        | Expr::Direction(_)
        | Expr::Var(_) => {}
    }
    Ok(())
}

fn require_command(command: &str, allowed_commands: &HashSet<String>) -> Result<(), Error> {
    if allowed_commands.contains(command) {
        Ok(())
    } else {
        Err(Error::CommandNotAllowed {
            command: command.to_string(),
        })
    }
}
