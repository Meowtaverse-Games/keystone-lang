#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expr),
    Move(Expr),
    Turn(Expr),
    Dig(Expr),
    Let(String, Expr),
    Loop(Expr, Vec<Statement>),
    While(Expr, Vec<Statement>),
    If(Expr, Vec<Statement>),
    Sleep(Expr),
    Receive(Expr),
    Send(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Uint(u32),
    Float(f32),
    String(String),
    Boolean(bool),
    Direction(Direction),
    Var(String),
    Call {
        callee: Callee,
        args: Vec<Box<Expr>>,
    },
    Unary {
        op: UnaryOp,
        exp: Box<Expr>,
    },
    Binary {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Uint,
    Float,
    String,
    Boolean,
    Direction,
    Side,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Back,
    Up,
    Down,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Callee {
    IsTouched,
    IsEmpty,
}
