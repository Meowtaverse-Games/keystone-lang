use crate::api::ExternalApi;
use crate::ast::{Callee, Direction, Expr, Op, Side, Statement, UnaryOp};
use crate::context::RuntimeContext;
use crate::error::Error;
use std::sync::Arc;

#[derive(Debug,Clone,PartialEq)]
pub enum Event{
    Print(String),
    Move(Direction),
    Turn(Side),
    Dig(Direction),
    Sleep(f32),
    Let,
    Tick
}

fn expr(input: Expr, ctx:&mut RuntimeContext, api:Arc<dyn ExternalApi + Send + Sync>) -> Result<Expr,Error>{
    match input{
        Expr::String(s) => Ok(Expr::String(String::from(s))),
        Expr::Uint(u) => Ok(Expr::Uint(u)),
        Expr::Float(f) => Ok(Expr::Float(f)),
        Expr::Boolean(b) => Ok(Expr::Boolean(b)),
        Expr::Direction(d) => Ok(Expr::Direction(d)),
        Expr::Var(s) => Ok(ctx.get(&s).clone()),
        Expr::Call { callee, args:_ } => {
            match callee{
                Callee::IsTouched => Ok(Expr::Boolean(api.is_touched())),
                Callee::IsEmpty => Ok(Expr::Boolean(api.is_empty()))
            }
        },
        Expr::Unary { op, exp } => {
            let x = expr(*exp, ctx, api)?;
            match op{
                UnaryOp::Not => {
                    match x {
                        Expr::Boolean(b) => Ok(Expr::Boolean(!b)),
                        _ => unreachable!()
                    }
                }
            }
        },
        Expr::Binary { op:o, lhs:l, rhs:r } => {
            let l = expr(*l, ctx, Arc::clone(&api))?;
            let r = expr(*r, ctx, Arc::clone(&api))?;
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
        Expr::Float(f) => {
            let mut s = format!("{:.6}",f).trim_end_matches('0').to_string();
            if s.ends_with('.') { s.push('0'); }
            s
        },
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

#[derive(Clone)]
pub struct EventIterator {
    stack: Vec<ExecutionFrame>,
    ctx: RuntimeContext,
    api: Arc<dyn ExternalApi + Send + Sync>
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
    pub fn new(statements: Vec<Statement>, ctx: RuntimeContext, api: Arc<dyn ExternalApi + Send + Sync>) -> Self {
        EventIterator {
            stack: vec![ExecutionFrame::Statement {
                statements: Arc::new(statements),
                index: 0,
            }],
            ctx,
            api
        }
    }

    fn process_statement(&mut self, stmt: Statement) -> Result<Option<Event>, Error> {
        match stmt {
            Statement::Print(x) => {
                let v = expr(x, &mut self.ctx, Arc::clone(&self.api))?;
                Ok(Some(Event::Print(stringize(&v))))
            },
            Statement::Move(x) => {
                let d = match expr(x, &mut self.ctx, Arc::clone(&self.api))? {
                    Expr::Direction(d) => d,
                    _ => unreachable!()
                };
                Ok(Some(Event::Move(d)))
            },
            Statement::Turn(x) => {
                let side = match x {
                    Expr::Direction(Direction::Left) => Side::Left,
                    Expr::Direction(Direction::Right) => Side::Right,
                    _ => unreachable!()
                };
                Ok(Some(Event::Turn(side)))
            },
            Statement::Dig(x) => {
                let d = match expr(x, &mut self.ctx, Arc::clone(&self.api))? {
                    Expr::Direction(d) => d,
                    _ => unreachable!()
                };
                Ok(Some(Event::Dig(d)))
            },
            Statement::Sleep(x) => {
                let f = match expr(x, &mut self.ctx, Arc::clone(&self.api))? {
                    Expr::Float(f) => f,
                    _ => unreachable!()
                };
                Ok(Some(Event::Sleep(f)))
            },
            Statement::Loop(x, body) => {
                let count = match expr(x, &mut self.ctx, Arc::clone(&self.api))? {
                    Expr::Uint(n) => n,
                    _ => unreachable!()
                };
                if count > 0 {
                    self.stack.push(ExecutionFrame::Loop {
                        count,
                        current: 0,
                        body: Arc::new(body),
                    });
                }
                Ok(None)
            },
            Statement::While(x, body) => {
                self.stack.push(ExecutionFrame::While {
                    condition: x,
                    body: Arc::new(body),
                });
                Ok(None)
            },
            Statement::If(x, body) => {
                let condition = match expr(x, &mut self.ctx, Arc::clone(&self.api))? {
                    Expr::Boolean(b) => b,
                    _ => unreachable!()
                };
                if condition {
                    self.stack.push(ExecutionFrame::Statement {
                        statements: Arc::new(body),
                        index: 0,
                    });
                }
                Ok(None)
            },
            Statement::Let(s, x) => {
                let rx = expr(x, &mut self.ctx, Arc::clone(&self.api))?;
                self.ctx.set(&s, rx);
                Ok(Some(Event::Let))
            }
        }
    }
}

impl Iterator for EventIterator {
    type Item = Result<Event, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let stack_len = self.stack.len();
            if stack_len == 0 {
                return None;
            }
            let frame_index = stack_len - 1;
            match &self.stack[frame_index] {
                ExecutionFrame::Statement { statements, index } => {
                    let current_index = *index;
                    if current_index >= statements.len() {
                        self.stack.pop();
                        continue;
                    }
                    let stmt = statements[current_index].clone();
                    if let ExecutionFrame::Statement { index, .. } = &mut self.stack[frame_index] {
                        *index = current_index + 1;
                    }
                    match self.process_statement(stmt) {
                        Ok(Some(event)) => return Some(Ok(event)),
                        Ok(None) => return Some(Ok(Event::Tick)),
                        Err(e) => return Some(Err(e)),
                    }
                },
                ExecutionFrame::Loop { count, current, body } => {
                    let current_iter = *current;
                    let total_count = *count;
                    let body_clone = Arc::clone(&body);
                    if current_iter >= total_count {
                        self.stack.pop();
                        continue;
                    }
                    if let ExecutionFrame::Loop { current, .. } = &mut self.stack[frame_index] {
                        *current = current_iter + 1;
                    }
                    self.stack.push(ExecutionFrame::Statement {
                        statements: body_clone,
                        index: 0,
                    });
                    continue;
                },
                ExecutionFrame::While { condition, body } => {
                    let body_clone = Arc::clone(&body);
                    match expr(condition.clone(), &mut self.ctx, Arc::clone(&self.api)) {
                        Ok(Expr::Boolean(true)) => {
                            self.stack.push(ExecutionFrame::Statement {
                                statements: body_clone,
                                index: 0,
                            });
                        }
                        Ok(Expr::Boolean(false)) => {
                            self.stack.pop();
                        }
                        _ => unreachable!(),
                    }
                    continue;
                }

            }
        }
    }
}

pub fn run(input: Vec<Statement>, ctx: RuntimeContext, api:Arc<dyn ExternalApi + Send + Sync>) -> EventIterator {
    EventIterator::new(input, ctx, api)
}