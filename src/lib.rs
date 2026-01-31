mod ast;
mod parser;
mod typecheck;
mod error;
mod vm;
mod context;

use chumsky::prelude::*;
use parser::program_parser as parser;
use typecheck::check;
use vm::run;
use context::{TypeContext,RuntimeContext};

pub use {error::Error, vm::{Event,EventIterator}, ast::{Direction,Side,Type,Op,UnaryOp},context::Builtins};

pub fn eval(input: &str, builtins: Builtins) -> Result<EventIterator, Error> {
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
    Ok(run(parsed.to_owned(), runtime_ctx,builtins))
}

pub fn eval_all(input: &str, builtins: Builtins) -> Result<Vec<Event>,Error> {
    let events: Result<Vec<Event>, Error> = eval(input,builtins)?.collect();
    events
}