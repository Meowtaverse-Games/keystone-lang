use chumsky::extra::Err;

use crate::ast::{Expr, Statement, Type};
use crate::error::*;


fn expr_check(input:&Expr) -> Result<Type,TypeError>{
    match input{
        Expr::String(_) => Ok(Type::String),
        Expr::Number(_) => Ok(Type::Number),
        Expr::Boolean(_) => Ok(Type::Boolean),
        Expr::Direction(_) => Ok(Type::Direction),
        Expr::Binary { op, lhs, rhs } => {
            let left = expr_check(lhs).unwrap();
            let right = expr_check(rhs).unwrap();
            if left != right {
                return Err(TypeError::MismatchedTypes { expected: left, found: right })
            }else{
                return Ok(left);
            }
        }
    }
}

pub fn check(input:&Vec<Statement>){
    let out = input.clone();
    // let mut res = Vec::<_>::new();
    for i in input{
        match i {
            Statement::Print(x) => {
                // expr_check(x)
            },
            Statement::Move(x) => {
                println!("ムーブ")
            },
            Statement::Turn(x) => {
                println!("ターン")
            }
        }
    }
}