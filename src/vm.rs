use crate::api::ExternalApi;
use crate::ast::{Callee, Direction, Expr, Op, Side, Statement, UnaryOp};
use crate::context::RuntimeContext;
use crate::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Print(String),
    Move(Direction),
    Turn(Side),
    Dig(Direction),
    Place(Direction),
    Sleep(f32),
    Let,
    Tick,
    Send(String),
    Receive(String),
    Wait,
}

fn expr(
    input: Expr,
    ctx: &mut RuntimeContext,
    api: Arc<dyn ExternalApi + Send + Sync>,
) -> Result<Expr, Error> {
    match input {
        Expr::String(s) => Ok(Expr::String(s)),
        Expr::Uint(u) => Ok(Expr::Uint(u)),
        Expr::Float(f) => Ok(Expr::Float(f)),
        Expr::Boolean(b) => Ok(Expr::Boolean(b)),
        Expr::Direction(d) => Ok(Expr::Direction(d)),
        Expr::Var(s) => Ok(ctx.get(&s).clone()),
        Expr::Call { callee, args } => match callee {
            Callee::IsTouched => Ok(Expr::Boolean(api.is_touched())),
            Callee::IsEmpty => {
                let a = expr(*args[0].clone(), ctx, Arc::clone(&api))?;
                if let Expr::Direction(d) = a {
                    Ok(Expr::Boolean(api.is_empty(d)))
                } else {
                    unreachable!()
                }
            }
            Callee::Rand => match args.len() {
                0 => {
                    let val: f32 = rand::random();
                    Ok(Expr::Float(val))
                }
                1 => {
                    let limit = match expr(*args[0].clone(), ctx, Arc::clone(&api))? {
                        Expr::Uint(n) => n,
                        _ => unreachable!(),
                    };
                    if limit == 0 {
                        Ok(Expr::Uint(0))
                    } else {
                        Ok(Expr::Uint(rand::random_range(0..limit)))
                    }
                }
                2 => {
                    let low = match expr(*args[0].clone(), ctx, Arc::clone(&api))? {
                        Expr::Uint(n) => n,
                        _ => unreachable!(),
                    };
                    let high = match expr(*args[1].clone(), ctx, Arc::clone(&api))? {
                        Expr::Uint(n) => n,
                        _ => unreachable!(),
                    };
                    if low >= high {
                        Ok(Expr::Uint(low))
                    } else {
                        Ok(Expr::Uint(rand::random_range(low..high)))
                    }
                }
                _ => unreachable!(),
            },
        },
        Expr::Unary { op, exp } => {
            let x = expr(*exp, ctx, api)?;
            if let (UnaryOp::Not, Expr::Boolean(b)) = (op, x) {
                Ok(Expr::Boolean(!b))
            } else {
                unreachable!()
            }
        }
        Expr::Binary {
            op: o,
            lhs: l,
            rhs: r,
        } => {
            let l = expr(*l, ctx, Arc::clone(&api))?;
            let r = expr(*r, ctx, Arc::clone(&api))?;
            match (l, r, o) {
                (Expr::Uint(x), Expr::Uint(y), Op::Add) => x
                    .checked_add(y)
                    .map(Expr::Uint)
                    .ok_or(Error::TooLargeNumber),
                (Expr::Uint(x), Expr::Uint(y), Op::Sub) => Ok(Expr::Uint(x - y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Mul) => x
                    .checked_mul(y)
                    .map(Expr::Uint)
                    .ok_or(Error::TooLargeNumber),
                (Expr::Uint(x), Expr::Uint(y), Op::Div) => {
                    if y == 0 {
                        Err(Error::ZeroDivisionError)
                    } else {
                        Ok(Expr::Uint(x / y))
                    }
                }
                (Expr::Uint(x), Expr::Uint(y), Op::Eq) => Ok(Expr::Boolean(x == y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Neq) => Ok(Expr::Boolean(x != y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Ge) => Ok(Expr::Boolean(x >= y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Le) => Ok(Expr::Boolean(x <= y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Gt) => Ok(Expr::Boolean(x > y)),
                (Expr::Uint(x), Expr::Uint(y), Op::Lt) => Ok(Expr::Boolean(x < y)),

                (Expr::Float(x), Expr::Float(y), Op::Add) => Ok(Expr::Float(x + y)),
                (Expr::Float(x), Expr::Float(y), Op::Sub) => Ok(Expr::Float(x - y)),
                (Expr::Float(x), Expr::Float(y), Op::Mul) => Ok(Expr::Float(x * y)),
                (Expr::Float(x), Expr::Float(y), Op::Div) => {
                    if y == 0. {
                        Err(Error::ZeroDivisionError)
                    } else {
                        Ok(Expr::Float(x / y))
                    }
                }
                (Expr::Float(x), Expr::Float(y), Op::Eq) => Ok(Expr::Boolean(x == y)),
                (Expr::Float(x), Expr::Float(y), Op::Neq) => Ok(Expr::Boolean(x != y)),
                (Expr::Float(x), Expr::Float(y), Op::Ge) => Ok(Expr::Boolean(x >= y)),
                (Expr::Float(x), Expr::Float(y), Op::Le) => Ok(Expr::Boolean(x <= y)),
                (Expr::Float(x), Expr::Float(y), Op::Gt) => Ok(Expr::Boolean(x > y)),
                (Expr::Float(x), Expr::Float(y), Op::Lt) => Ok(Expr::Boolean(x < y)),

                (Expr::String(x), Expr::String(y), Op::Add) => Ok(Expr::String(x + &y)),
                (Expr::String(x), Expr::String(y), Op::Eq) => Ok(Expr::Boolean(x == y)),
                (Expr::String(x), Expr::String(y), Op::Neq) => Ok(Expr::Boolean(x != y)),

                (Expr::Boolean(x), Expr::Boolean(y), Op::And) => Ok(Expr::Boolean(x && y)),
                (Expr::Boolean(x), Expr::Boolean(y), Op::Or) => Ok(Expr::Boolean(x || y)),
                (Expr::Boolean(x), Expr::Boolean(y), Op::Eq) => Ok(Expr::Boolean(x == y)),
                (Expr::Boolean(x), Expr::Boolean(y), Op::Neq) => Ok(Expr::Boolean(x != y)),

                (Expr::Direction(x), Expr::Direction(y), Op::Eq) => Ok(Expr::Boolean(x == y)),
                (Expr::Direction(x), Expr::Direction(y), Op::Neq) => Ok(Expr::Boolean(x != y)),

                _ => unreachable!(),
            }
        }
    }
}

fn stringize(input: &Expr) -> String {
    match input {
        Expr::String(e) => e.to_owned(),
        Expr::Uint(u) => u.to_string(),
        Expr::Float(f) => {
            let mut s = format!("{:.6}", f).trim_end_matches('0').to_string();
            if s.ends_with('.') {
                s.push('0');
            }
            s
        }
        Expr::Boolean(b) => b.to_string(),
        Expr::Direction(d) => match d {
            Direction::Forward => "Forward",
            Direction::Back => "Back",
            Direction::Left => "Left",
            Direction::Right => "Right",
            Direction::Up => "Up",
            Direction::Down => "Down",
        }
        .to_owned(),
        _ => unreachable!(),
    }
}

#[derive(Clone)]
pub struct EventIterator {
    stack: Vec<ExecutionFrame>,
    ctx: RuntimeContext,
    api: Arc<dyn ExternalApi + Send + Sync>,
}

#[derive(Clone)]
enum ExecutionFrame {
    Statement {
        statements: Arc<Vec<Statement>>,
        index: usize,
    },
    Loop {
        count: u32,
        current: u32,
        body: Arc<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Arc<Vec<Statement>>,
    },
}

impl EventIterator {
    pub fn new(
        statements: Vec<Statement>,
        ctx: RuntimeContext,
        api: Arc<dyn ExternalApi + Send + Sync>,
    ) -> Self {
        EventIterator {
            stack: vec![ExecutionFrame::Statement {
                statements: Arc::new(statements),
                index: 0,
            }],
            ctx,
            api,
        }
    }

    fn process_statement(&mut self, stmt: Statement) -> Result<Option<Event>, Error> {
        let mut exec = |x| expr(x, &mut self.ctx, Arc::clone(&self.api));
        match stmt {
            Statement::Print(x) => {
                let v = exec(x)?;
                Ok(Some(Event::Print(stringize(&v))))
            }
            Statement::Move(x) => {
                if let Expr::Direction(d) = exec(x)? {
                    Ok(Some(Event::Move(d)))
                } else {
                    unreachable!()
                }
            }
            Statement::Turn(x) => {
                if let Expr::Direction(d) = exec(x)? {
                    let side = match d {
                        Direction::Left => Side::Left,
                        Direction::Right => Side::Right,
                        _ => unreachable!(),
                    };
                    Ok(Some(Event::Turn(side)))
                } else {
                    unreachable!()
                }
            }
            Statement::Dig(x) => {
                if let Expr::Direction(d) = exec(x)? {
                    Ok(Some(Event::Dig(d)))
                } else {
                    unreachable!()
                }
            }
            Statement::Place(x) => {
                if let Expr::Direction(d) = exec(x)? {
                    Ok(Some(Event::Place(d)))
                } else {
                    unreachable!()
                }
            }
            Statement::Sleep(x) => {
                if let Expr::Float(f) = exec(x)? {
                    Ok(Some(Event::Sleep(f)))
                } else {
                    unreachable!()
                }
            }
            Statement::Loop(x, body) => {
                if let Expr::Uint(count) = exec(x)? {
                    if count > 0 {
                        self.stack.push(ExecutionFrame::Loop {
                            count,
                            current: 0,
                            body: Arc::new(body),
                        });
                    }
                    Ok(None)
                } else {
                    unreachable!()
                }
            }
            Statement::While(x, body) => {
                self.stack.push(ExecutionFrame::While {
                    condition: x,
                    body: Arc::new(body),
                });
                Ok(None)
            }
            Statement::If(x, body) => {
                if let Expr::Boolean(condition) = exec(x)? {
                    if condition {
                        self.stack.push(ExecutionFrame::Statement {
                            statements: Arc::new(body),
                            index: 0,
                        });
                    }
                    Ok(None)
                } else {
                    unreachable!()
                }
            }
            Statement::Let(s, x) => {
                let rx = exec(x)?;
                self.ctx.set(&s, rx);
                Ok(Some(Event::Let))
            }
            Statement::Send(x) => {
                let s = stringize(&exec(x)?);
                self.api.send_signal(&s);
                Ok(Some(Event::Send(s)))
            }
            Statement::Receive(x) => {
                let s = stringize(&exec(x)?);
                let event = if self.api.receive_signal(&s) {
                    Event::Receive(s)
                } else {
                    Event::Wait
                };
                Ok(Some(event))
            }
        }
    }
}

impl Iterator for EventIterator {
    type Item = Result<Event, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            match frame {
                ExecutionFrame::Statement { statements, index } => {
                    if *index >= statements.len() {
                        self.stack.pop();
                        continue;
                    }
                    let stmt = statements[*index].clone();
                    *index += 1;
                    match self.process_statement(stmt) {
                        Ok(Some(Event::Wait)) => {
                            if let Some(ExecutionFrame::Statement { index, .. }) =
                                self.stack.last_mut()
                            {
                                *index -= 1;
                            }
                            return Some(Ok(Event::Wait));
                        }
                        Ok(Some(event)) => return Some(Ok(event)),
                        Ok(None) => return Some(Ok(Event::Tick)),
                        Err(e) => return Some(Err(e)),
                    }
                }
                ExecutionFrame::Loop {
                    count,
                    current,
                    body,
                } => {
                    if *current >= *count {
                        self.stack.pop();
                        continue;
                    }
                    *current += 1;
                    let body_clone = Arc::clone(body);
                    self.stack.push(ExecutionFrame::Statement {
                        statements: body_clone,
                        index: 0,
                    });
                    continue;
                }
                ExecutionFrame::While { condition, body } => {
                    let body_clone = Arc::clone(body);
                    let cond_res = expr(condition.clone(), &mut self.ctx, Arc::clone(&self.api));
                    if let Ok(Expr::Boolean(true)) = cond_res {
                        self.stack.push(ExecutionFrame::Statement {
                            statements: body_clone,
                            index: 0,
                        });
                    } else {
                        self.stack.pop();
                    }
                    continue;
                }
            }
        }
    }
}

pub fn run(
    input: Vec<Statement>,
    ctx: RuntimeContext,
    api: Arc<dyn ExternalApi + Send + Sync>,
) -> EventIterator {
    EventIterator::new(input, ctx, api)
}
