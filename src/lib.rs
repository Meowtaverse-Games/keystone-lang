mod ast;
mod parser;
mod typecheck;
mod error;
mod vm;
mod context;
mod api;

use chumsky::prelude::*;
use parser::program_parser as parser;
use typecheck::check;
use vm::run;
use context::{TypeContext,RuntimeContext};

pub use {error::Error, vm::{Event,EventIterator}, ast::{Direction,Side,Type,Op,UnaryOp}, api::ExternalApi};

pub fn eval<'a>(input: &'a str, api:&'a dyn ExternalApi) -> Result<EventIterator<'a>, Error> {
    let parsed = parser().parse(input.trim());
    let Some(parsed) = parsed.output() else {
        let mut msg:Vec<String> = Vec::new();
        for err in parsed.errors(){
            msg.push(err.to_string());
        }
        return Err(Error::SyntaxError{ messages:msg });
    };
    let mut type_ctx = TypeContext::new();
    check(parsed, &mut type_ctx)?;
    let runtime_ctx = RuntimeContext::new();
    Ok(run(parsed.to_owned(), runtime_ctx,api))
}

pub fn eval_all<'a>(input: &'a str, api:&'a dyn ExternalApi) -> Result<Vec<Event>,Error> {
    let events: Result<Vec<Event>, Error> = eval(input,api)?.collect();
    events
}