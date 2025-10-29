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
use context::TypeContext;

use crate::{context::RuntimeContext, error::Error, vm::Event};

pub fn eval(input: &'static str) -> Result<Vec<Event>,Error> {
    let parsed = parser().parse(input.trim());
    let Some(parsed) = parsed.output() else {
        let mut msg:Vec<String> = Vec::new();
        for err in parsed.errors(){
            msg.push(err.to_string());
        }
        return Err(Error::SyntaxError{ messages:msg });
    };
    // println!("{:?}",parsed);
    // println!("{:?}",checked);
    let mut type_ctx = TypeContext::new();
    check(parsed, &mut type_ctx)?;
    let mut runtime_ctx = RuntimeContext::new();
    let events = run(parsed.to_owned(), &mut runtime_ctx);
    return events
}
