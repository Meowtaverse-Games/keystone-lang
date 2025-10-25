use crate::ast::{Expr, Op, Statement, Type};
use crate::error::*;


fn expr_check(input:&Expr) -> Result<Type,TypeError>{
    match input{
        Expr::String(_) => Ok(Type::String),
        Expr::Number(_) => Ok(Type::Number),
        Expr::Boolean(_) => Ok(Type::Boolean),
        Expr::Direction(_) => Ok(Type::Direction),
        Expr::Var(_) => Ok(Type::Var),
        Expr::Binary { op, lhs, rhs } => {
            let left = expr_check(lhs)?;
            let right = expr_check(rhs)?;
            if left != right {
                return Err(TypeError::MismatchedTypes { op:op.clone(), left, right })
            }
            let typ = left;
            match typ{
                Type::Number => match op{
                        Op::Add|Op::Sub|Op::Mul|Op::Div => Ok(Type::Number),
                        Op::Eq|Op::Neq|Op::Ge|Op::Le|Op::Gt|Op::Lt => Ok(Type::Boolean),
                        _ => Err(TypeError::InvalidOperandType { op: op.clone(), typ: Type::Number })
                },
                Type::String => match op{
                        Op::Add => Ok(Type::String),
                        Op::Eq|Op::Neq => Ok(Type::Boolean),
                        _ => Err(TypeError::InvalidOperandType { op: op.clone(), typ: Type::String })
                },
                Type::Boolean => match op{
                        Op::And|Op::Or => Ok(Type::Boolean),
                        _ => Err(TypeError::InvalidOperandType { op: op.clone(), typ: Type::Boolean })
                },
                other => Err(TypeError::InvalidOperandType { op: op.clone(), typ: other })
            }
        }
    }
}

pub fn check(input:&Vec<Statement>) -> Result<(),TypeError>{
    for i in input{
        match i {
            Statement::Print(x) => {
                expr_check(x)?;
            },
            Statement::Move(x) => {
                let t = expr_check(x)?;
                if !matches!(t,Type::Direction|Type::Var) {
                    return Err(TypeError::UnexpectedType { statement: String::from("Move"), found_type: t });
                }
            },
            Statement::Turn(x) => {
                let t = expr_check(x)?;
                if !matches!(t,Type::Direction|Type::Var) {
                    return Err(TypeError::UnexpectedType { statement: String::from("Turn"), found_type: t });
                }
            },
            Statement::Let(_,x) => {
                expr_check(x)?;
            },
            Statement::Loop(x, y) => {
                let t = expr_check(x)?;
                if !matches!(t,Type::Number|Type::Var){
                    return Err(TypeError::UnexpectedType { statement: String::from("Loop"), found_type: t });
                } else{
                    check(y)?
                }
            },
            Statement::If(x, y) => {
                let t = expr_check(x)?;
                if !matches!(t,Type::Boolean|Type::Var){
                    return Err(TypeError::UnexpectedType { statement: String::from("If"), found_type: t });
                } else{
                    check(y)?
                }
            }
        }
    }
    Ok(())
}