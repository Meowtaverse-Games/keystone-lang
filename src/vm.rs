use crate::ast::{Direction, Expr, Op, Side, Statement};
use crate::context::RuntimeContext;
use crate::error::Error;

#[derive(Debug)]
pub enum Event{
    Print(String),
    Move(Direction),
    Turn(Side),
    Let
}

fn expr(input: Expr, ctx:&mut RuntimeContext) -> Result<Expr,Error>{
    match input{
        Expr::String(s) => Ok(Expr::String(String::from(s))),
        Expr::Number(n) => Ok(Expr::Number(n)),
        Expr::Boolean(b) => Ok(Expr::Boolean(b)),
        Expr::Direction(d) => Ok(Expr::Direction(d)),
        Expr::Var(s) => Ok(ctx.get(&s).clone()),
        Expr::Binary { op:o, lhs:l, rhs:r } => {
            let l = expr(*l, ctx)?;
            let r = expr(*r, ctx)?;
            match (l,r){
                (Expr::Number(x),Expr::Number(y)) => {
                    match o{
                        Op::Add => Ok(Expr::Number(x+y)),
                        Op::Sub => Ok(Expr::Number(x-y)),
                        Op::Mul => Ok(Expr::Number(x*y)),
                        Op::Div => {
                            if y == 0 { Err(Error::ZeroDivisionError) }
                            else { Ok(Expr::Number(x/y)) }
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
                _ => unreachable!()
            }
        }
    }
}

fn stringize(input: &Expr) -> String{
    match input{
        Expr::String(e) => String::from(e),
        Expr::Number(n) => n.to_string(),
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
        Statement::Loop(x, vs) => {
            let xn = expr(x,ctx)?;
            if let Expr::Number(n) = xn{
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
            ctx.set(&s, rx);
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