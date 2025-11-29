use crate::ast::{Direction, Expr, Op, Side, Statement};
use crate::context::RuntimeContext;
use crate::error::Error;

#[derive(Debug,Clone,PartialEq)]
pub enum Event{
    Print(String),
    Move(Direction),
    Turn(Side),
    Sleep(f32),
    Let
}

fn expr(input: Expr, ctx:&mut RuntimeContext) -> Result<Expr,Error>{
    match input{
        Expr::String(s) => Ok(Expr::String(String::from(s))),
        Expr::Uint(u) => Ok(Expr::Uint(u)),
        Expr::Float(f) => Ok(Expr::Float(f)),
        Expr::Boolean(b) => Ok(Expr::Boolean(b)),
        Expr::Direction(d) => Ok(Expr::Direction(d)),
        Expr::Var(s) => Ok(ctx.get(&s).clone()),
        Expr::Binary { op:o, lhs:l, rhs:r } => {
            let l = expr(*l, ctx)?;
            let r = expr(*r, ctx)?;
            match (l,r){
                (Expr::Uint(x),Expr::Uint(y)) => {
                    match o{
                        Op::Add => {
                            let opt = x.checked_add(y);
                            if let Some(n) = opt{
                                Ok(Expr::Uint(n))
                            }else {
                                Err(Error::TooLargeNumber)
                            }
                        },
                        Op::Sub => Ok(Expr::Uint(x-y)),
                        Op::Mul => {
                            let opt = x.checked_mul(y);
                            if let Some(n) = opt{
                                Ok(Expr::Uint(n))
                            }else {
                                Err(Error::TooLargeNumber)
                            }
                        },
                        Op::Div => {
                            if y == 0 { Err(Error::ZeroDivisionError) }
                            else { Ok(Expr::Uint(x/y)) }
                        },
                        Op::Eq => Ok(Expr::Boolean(x==y)),
                        Op::Neq => Ok(Expr::Boolean(x != y)),
                        Op::Ge => Ok(Expr::Boolean(x >= y)),
                        Op::Le => Ok(Expr::Boolean(x <= y)),
                        Op::Gt => Ok(Expr::Boolean(x > y)),
                        Op::Lt => Ok(Expr::Boolean(x < y)),
                        _ => unreachable!()
                    }
                },
                (Expr::Float(x),Expr::Float(y)) => {
                    match o{
                        Op::Add => Ok(Expr::Float(x+y)),
                        Op::Sub => Ok(Expr::Float(x-y)),
                        Op::Mul => Ok(Expr::Float(x*y)),
                        Op::Div => {
                            if y == 0. { Err(Error::ZeroDivisionError) }
                            else { Ok(Expr::Float(x/y)) }
                        },
                        Op::Eq => Ok(Expr::Boolean(x==y)),
                        Op::Neq => Ok(Expr::Boolean(x != y)),
                        Op::Ge => Ok(Expr::Boolean(x >= y)),
                        Op::Le => Ok(Expr::Boolean(x <= y)),
                        Op::Gt => Ok(Expr::Boolean(x > y)),
                        Op::Lt => Ok(Expr::Boolean(x < y)),
                        _ => unreachable!()
                    }
                },
                (Expr::String(x),Expr::String(y)) => {
                    match o{
                        Op::Add => Ok(Expr::String(x+&y)),
                        Op::Eq => Ok(Expr::Boolean(x==y)),
                        Op::Neq => Ok(Expr::Boolean(x != y)),
                        _ => unreachable!()
                    }
                },
                (Expr::Boolean(x),Expr::Boolean(y)) => {
                    match o{
                        Op::And => Ok(Expr::Boolean(x&&y)),
                        Op::Or => Ok(Expr::Boolean(x||y)),
                        Op::Eq => Ok(Expr::Boolean(x==y)),
                        Op::Neq => Ok(Expr::Boolean(x != y)),
                        _ => unreachable!()
                    }
                },
                (Expr::Direction(x),Expr::Direction(y)) => {
                    match o{
                        Op::Eq => Ok(Expr::Boolean(x==y)),
                        Op::Neq => Ok(Expr::Boolean(x != y)),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }
    }
}

fn stringize(input: &Expr) -> String{
    match input{
        Expr::String(e) => String::from(e),
        Expr::Uint(u) => u.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::Boolean(b) => b.to_string(),
        Expr::Direction(d) => String::from(match d{
            Direction::Forward => "Forward",
            Direction::Back => "Back",
            Direction::Left => "Left",
            Direction::Right => "Right",
            Direction::Up => "Up",
            Direction::Down => "Down"
        }),
        _ => unreachable!()
    }
}

fn statement(input: Statement, ctx:&mut RuntimeContext) -> Result<Vec<Event>,Error>{
    match input{
        Statement::Print(x) => {
            let v = expr(x, ctx)?;
            Ok(vec![Event::Print(stringize(&v))])
        },
        Statement::Move(x) => Ok(vec![Event::Move(match expr(x, ctx)? {
            Expr::Direction(d) => d,
            _ => unreachable!()
        })]),
        Statement::Turn(x) => Ok(vec![Event::Turn(match x {
            Expr::Direction(Direction::Left) => Side::Left,
            Expr::Direction(Direction::Right) => Side::Right,
            _ => unreachable!()
        })]),
        Statement::Sleep(x) => Ok(vec![Event::Sleep(match expr(x, ctx)? {
            Expr::Float(f) => f,
            _ => unreachable!()
        })]),
        Statement::Loop(x, vs) => {
            let xn = expr(x,ctx)?;
            if let Expr::Uint(n) = xn{
                let mut events:Vec<Event> = Vec::new();
                for _ in 0..n {
                    for i in &vs{
                        events.extend(statement(i.clone(), ctx)?);
                    }
                }
                Ok(events)
            }else{
                unreachable!()
            }
        },
        Statement::If(x, vs) => {
            let xb = expr(x,ctx)?;
            if let Expr::Boolean(b) = xb{
                let mut events:Vec<Event> = Vec::new();
                if b {
                    for i in &vs{
                        events.extend(statement(i.clone(), ctx)?);
                    }
                }
                Ok(events)
            }else{
                unreachable!()
            }
        },
        Statement::Let(s, x) => {
            let rx = expr(x,ctx)?;
            ctx.set(&s, rx.clone());
            Ok(vec![Event::Let])
        }
    }
}

pub fn run(input:Vec<Statement>, ctx:&mut RuntimeContext)->Result<Vec<Event>,Error>{
    let mut events:Vec<Event> = Vec::new();
    for i in input{
        match statement(i, ctx){
            Ok(mut r) => {events.append(&mut r)},
            Err(e) => {return Err(e)}
        }
    }
    Ok(events)
}