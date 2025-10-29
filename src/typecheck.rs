use crate::ast::{Expr, Op, Statement, Type};
use crate::context::TypeContext;
use crate::error::*;


fn expr_check(input:&Expr, ctx: &mut TypeContext) -> Result<Type,Error>{
    match input{
        Expr::String(_) => Ok(Type::String),
        Expr::Number(_) => Ok(Type::Number),
        Expr::Boolean(_) => Ok(Type::Boolean),
        Expr::Direction(_) => Ok(Type::Direction),
        Expr::Var(s) => {
            let ot = ctx.get(s);
            match ot{
                Some(t) => Ok(t.clone()),
                None => Err(Error::NameError { name: s.to_owned() })
            }
        },
        Expr::Binary { op, lhs, rhs } => {
            let left = expr_check(lhs,ctx)?;
            let right = expr_check(rhs,ctx)?;
            if left != right {
                return Err(Error::MismatchedTypes { op:op.clone(), left, right })
            }
            let typ = left;
            match typ{
                Type::Number => match op{
                        Op::Add|Op::Sub|Op::Mul|Op::Div => Ok(Type::Number),
                        Op::Eq|Op::Neq|Op::Ge|Op::Le|Op::Gt|Op::Lt => Ok(Type::Boolean),
                        _ => Err(Error::InvalidOperandType { op: op.clone(), typ: Type::Number })
                },
                Type::String => match op{
                        Op::Add => Ok(Type::String),
                        Op::Eq|Op::Neq => Ok(Type::Boolean),
                        _ => Err(Error::InvalidOperandType { op: op.clone(), typ: Type::String })
                },
                Type::Boolean => match op{
                        Op::And|Op::Or => Ok(Type::Boolean),
                        Op::Eq|Op::Neq => Ok(Type::Boolean),
                        _ => Err(Error::InvalidOperandType { op: op.clone(), typ: Type::Boolean })
                }
                other => Err(Error::InvalidOperandType { op: op.clone(), typ: other })
            }
        }
    }
}

pub fn check(input:&Vec<Statement>, ctx: &mut TypeContext) -> Result<(),Error>{
    for i in input{
        match i {
            Statement::Print(x) => {
                expr_check(x, ctx)?;
            },
            Statement::Move(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t,Type::Direction) {
                    return Err(Error::UnexpectedType { statement: String::from("Move"), found_type: t });
                }
            },
            Statement::Turn(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t,Type::Direction) {
                    return Err(Error::UnexpectedType { statement: String::from("Turn"), found_type: t });
                }
            },
            Statement::Let(s,x) => {
                let t = expr_check(x, ctx)?;
                ctx.set(s,t);
            },
            Statement::Loop(x, y) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t,Type::Number){
                    return Err(Error::UnexpectedType { statement: String::from("Loop"), found_type: t });
                } else{
                    check(y,ctx)?
                }
            },
            Statement::If(x, y) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t,Type::Boolean){
                    return Err(Error::UnexpectedType { statement: String::from("If"), found_type: t });
                } else{
                    check(y,ctx)?
                }
            }
        }
    }
    Ok(())
}