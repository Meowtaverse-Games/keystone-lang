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
        Expr::Var(s) => {
            let ot = ctx.get(s);
            match ot {
                Some(t) => Ok(t.clone()),
                None => Err(Error::NameError { name: s.to_owned() }),
            }
        }
        Expr::Call { callee, args } => {
            let len = args.len() as u8;
            match callee {
                Callee::IsTouched => {
                    if len == 0 {
                        Ok(Type::Boolean)
                    } else {
                        Err(Error::ArgError {
                            called: String::from("is_touched()"),
                            expected: 0,
                            got: len,
                        })
                    }
                }
                Callee::IsEmpty => {
                    if len == 1 {
                        let t = expr_check(&args[0], ctx)?;
                        match t {
                            Type::Direction => Ok(Type::Boolean),
                            _ => Err(Error::UnexpectedType {
                                statement: String::from("Turn"),
                                found_type: t,
                            }),
                        }
                    } else {
                        Err(Error::ArgError {
                            called: String::from("is_empty()"),
                            expected: 1,
                            got: len,
                        })
                    }
                }
            }
        }
        Expr::Unary { op, exp } => {
            let typ = expr_check(exp, ctx)?;
            match typ {
                Type::Boolean => Ok(Type::Boolean),
                _ => Err(Error::InvalidUnaryOperandType {
                    op: op.clone(),
                    typ,
                }),
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
            let typ = left;
            match typ {
                Type::Uint => match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div => Ok(Type::Uint),
                    Op::Eq | Op::Neq | Op::Ge | Op::Le | Op::Gt | Op::Lt => Ok(Type::Boolean),
                    _ => Err(Error::InvalidOperandType {
                        op: op.clone(),
                        typ: Type::Uint,
                    }),
                },
                Type::Float => match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div => Ok(Type::Float),
                    Op::Eq | Op::Neq | Op::Ge | Op::Le | Op::Gt | Op::Lt => Ok(Type::Boolean),
                    _ => Err(Error::InvalidOperandType {
                        op: op.clone(),
                        typ: Type::Float,
                    }),
                },
                Type::String => match op {
                    Op::Add => Ok(Type::String),
                    Op::Eq | Op::Neq => Ok(Type::Boolean),
                    _ => Err(Error::InvalidOperandType {
                        op: op.clone(),
                        typ: Type::String,
                    }),
                },
                Type::Boolean => match op {
                    Op::And | Op::Or => Ok(Type::Boolean),
                    Op::Eq | Op::Neq => Ok(Type::Boolean),
                    _ => Err(Error::InvalidOperandType {
                        op: op.clone(),
                        typ: Type::Boolean,
                    }),
                },
                Type::Direction => match op {
                    Op::Eq | Op::Neq => Ok(Type::Boolean),
                    _ => Err(Error::InvalidOperandType {
                        op: op.clone(),
                        typ: Type::Direction,
                    }),
                },
                other => Err(Error::InvalidOperandType {
                    op: op.clone(),
                    typ: other,
                }),
            }
        }
    }
}

pub fn check(input: &Vec<Statement>, ctx: &mut TypeContext) -> Result<(), Error> {
    for i in input {
        match i {
            Statement::Print(x) => {
                expr_check(x, ctx)?;
            }
            Statement::Move(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Direction) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("Move"),
                        found_type: t,
                    });
                }
            }
            Statement::Turn(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Direction) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("Turn"),
                        found_type: t,
                    });
                }
            }
            Statement::Dig(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Direction) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("Dig"),
                        found_type: t,
                    });
                }
            }
            Statement::Sleep(x) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Float) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("Sleep"),
                        found_type: t,
                    });
                }
            }
            Statement::Let(s, x) => {
                let t = expr_check(x, ctx)?;
                ctx.set(s, t);
            }
            Statement::Loop(x, y) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Uint) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("Loop"),
                        found_type: t,
                    });
                } else {
                    check(y, ctx)?
                }
            }
            Statement::While(x, y) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Boolean) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("While"),
                        found_type: t,
                    });
                } else {
                    check(y, ctx)?
                }
            }
            Statement::If(x, y) => {
                let t = expr_check(x, ctx)?;
                if !matches!(t, Type::Boolean) {
                    return Err(Error::UnexpectedType {
                        statement: String::from("If"),
                        found_type: t,
                    });
                } else {
                    check(y, ctx)?
                }
            }
        }
    }
    Ok(())
}
