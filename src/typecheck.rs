use crate::ast::{Callee, Expr, Op, Statement, Type};
use crate::context::TypeContext;
use crate::error::*;

fn expr_check(input: &Expr, ctx: &mut TypeContext) -> Result<Type, Error> {
    match input {
        Expr::String(_) => Ok(Type::String),
        Expr::Uint(_) => Ok(Type::Uint),
        Expr::Float(_) => Ok(Type::Float),
        Expr::Boolean(_) => Ok(Type::Boolean),
        Expr::Direction(_) => Ok(Type::Direction),
        Expr::Var(s) => ctx
            .get(s)
            .cloned()
            .ok_or_else(|| Error::NameError { name: s.to_owned() }),
        Expr::Call { callee, args } => {
            let len = args.len() as u8;
            match (callee, len) {
                (Callee::IsTouched, 0) => Ok(Type::Boolean),
                (Callee::IsTouched, _) => Err(Error::ArgError {
                    called: String::from("is_touched()"),
                    expected: 0,
                    got: len,
                }),
                (Callee::IsEmpty, 1) => {
                    ensure_type(expr_check(&args[0], ctx)?, Type::Direction, "is_empty()")?;
                    Ok(Type::Boolean)
                }
                (Callee::IsEmpty, _) => Err(Error::ArgError {
                    called: String::from("is_empty()"),
                    expected: 1,
                    got: len,
                }),
                (Callee::Rand, 0) => Ok(Type::Float),
                (Callee::Rand, 1) => {
                    ensure_type(expr_check(&args[0], ctx)?, Type::Uint, "rand()")?;
                    Ok(Type::Uint)
                }
                (Callee::Rand, 2) => {
                    ensure_type(expr_check(&args[0], ctx)?, Type::Uint, "rand()")?;
                    ensure_type(expr_check(&args[1], ctx)?, Type::Uint, "rand()")?;
                    Ok(Type::Uint)
                }
                (Callee::Rand, _) => Err(Error::ArgError {
                    called: String::from("rand()"),
                    expected: 2,
                    got: len,
                }),
            }
        }
        Expr::Unary { op, exp } => {
            let typ = expr_check(exp, ctx)?;
            if let Type::Boolean = typ {
                Ok(Type::Boolean)
            } else {
                Err(Error::InvalidUnaryOperandType {
                    op: op.clone(),
                    typ,
                })
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let left = expr_check(lhs, ctx)?;
            let right = expr_check(rhs, ctx)?;
            if left != right {
                return Err(Error::MismatchedTypes {
                    op: op.clone(),
                    left,
                    right,
                });
            }
            match (left, op) {
                (Type::Uint, Op::Add | Op::Sub | Op::Mul | Op::Div) => Ok(Type::Uint),
                (Type::Uint, Op::Eq | Op::Neq | Op::Ge | Op::Le | Op::Gt | Op::Lt) => {
                    Ok(Type::Boolean)
                }
                (Type::Float, Op::Add | Op::Sub | Op::Mul | Op::Div) => Ok(Type::Float),
                (Type::Float, Op::Eq | Op::Neq | Op::Ge | Op::Le | Op::Gt | Op::Lt) => {
                    Ok(Type::Boolean)
                }
                (Type::String, Op::Add) => Ok(Type::String),
                (Type::String, Op::Eq | Op::Neq) => Ok(Type::Boolean),
                (Type::Boolean, Op::And | Op::Or) => Ok(Type::Boolean),
                (Type::Boolean, Op::Eq | Op::Neq) => Ok(Type::Boolean),
                (Type::Direction, Op::Eq | Op::Neq) => Ok(Type::Boolean),
                (typ, _) => Err(Error::InvalidOperandType {
                    op: op.clone(),
                    typ,
                }),
            }
        }
    }
}

fn ensure_type(actual: Type, expected: Type, name: &str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::UnexpectedType {
            statement: String::from(name),
            found_type: actual,
        })
    }
}
fn ensure_types(actual: Type, expected_list: &[Type], name: &str) -> Result<(), Error> {
    if expected_list.contains(&actual) {
        Ok(())
    } else {
        Err(Error::UnexpectedType {
            statement: String::from(name),
            found_type: actual,
        })
    }
}

pub fn check(input: &[Statement], ctx: &mut TypeContext) -> Result<(), Error> {
    for i in input {
        match i {
            Statement::Print(x) => {
                expr_check(x, ctx)?;
            }
            Statement::Move(x) => ensure_type(expr_check(x, ctx)?, Type::Direction, "Move")?,
            Statement::Turn(x) => ensure_type(expr_check(x, ctx)?, Type::Direction, "Turn")?,
            Statement::Dig(x) => ensure_type(expr_check(x, ctx)?, Type::Direction, "Dig")?,
            Statement::Sleep(x) => ensure_type(expr_check(x, ctx)?, Type::Float, "Sleep")?,
            Statement::Let(s, x) => {
                let t = expr_check(x, ctx)?;
                ctx.set(s, t);
            }
            Statement::Loop(x, y) => {
                ensure_type(expr_check(x, ctx)?, Type::Uint, "Loop")?;
                check(y, ctx)?;
            }
            Statement::While(x, y) => {
                ensure_type(expr_check(x, ctx)?, Type::Boolean, "While")?;
                check(y, ctx)?;
            }
            Statement::If(x, y) => {
                ensure_type(expr_check(x, ctx)?, Type::Boolean, "If")?;
                check(y, ctx)?;
            }
            Statement::Send(x) => {
                ensure_types(expr_check(x, ctx)?, &[Type::Uint, Type::String], "Send")?
            }
            Statement::Receive(x) => {
                ensure_types(expr_check(x, ctx)?, &[Type::Uint, Type::String], "Receive")?
            }
        }
    }
    Ok(())
}
