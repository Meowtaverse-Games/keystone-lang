#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expr),
    Move(Expr),
    Turn(Expr)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(u32),
    String(String),
    Boolean(bool),
    Direction(Direction),
    Binary {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,String,Boolean,Direction,Side
}

#[derive(Debug, Clone)]
pub enum Op {
    Add, Sub, Mul, Div,
    Eq, Neq, Lt, Gt, Le, Ge,
    And,Or
}
#[derive(Debug, Clone)]
pub enum Direction {
    Left,Right,Forward,Back,Up,Down
}
#[derive(Debug, Clone)]
pub enum Side{
    Left,Right
}