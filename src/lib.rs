mod api;
mod ast;
mod context;
mod error;
mod parser;
mod typecheck;
mod vm;

use std::sync::Arc;

use chumsky::prelude::*;
use context::{RuntimeContext, TypeContext};
use parser::program_parser as parser;
use typecheck::check;
use vm::run;

pub use {
    api::ExternalApi,
    ast::{Direction, Expr, Op, Side, Statement, Type, UnaryOp},
    error::Error,
    vm::{Event, EventIterator},
};

pub fn eval(input: &str, api: Arc<dyn ExternalApi + Send + Sync>) -> Result<EventIterator, Error> {
    let parsed = parser().parse(input.trim());
    let Some(parsed) = parsed.output() else {
        let mut msg: Vec<String> = Vec::new();
        for err in parsed.errors() {
            msg.push(err.to_string());
        }
        return Err(Error::SyntaxError { messages: msg });
    };
    let mut type_ctx = TypeContext::new();
    check(parsed, &mut type_ctx)?;
    let runtime_ctx = RuntimeContext::new();
    Ok(run(parsed.to_owned(), runtime_ctx, api))
}

pub fn eval_all(input: &str, api: Arc<dyn ExternalApi + Send + Sync>) -> Result<Vec<Event>, Error> {
    let events: Result<Vec<Event>, Error> = eval(input, api)?.collect();
    events
}
