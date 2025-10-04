mod ast;
mod parser;
mod typecheck;
mod error;

use chumsky::prelude::*;
use parser::program_parser as parser;
use typecheck::check;

pub fn run(input: &'static str) {
    let parsed = parser().parse(input.trim());
    let Some(parsed) = parsed.output() else {
        for err in parsed.errors(){
            println!("{}",err);
        }
        return;
    };
    let checked = check(parsed);
}
