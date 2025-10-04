use chumsky::prelude::*;
use crate::ast::*;

fn expr_parser<'a>() -> impl Parser<'a, &'a str, Expr, extra::Err<Simple<'a, char>>> {
    recursive(|expr| {
        let boolean = just("true")
            .to(Expr::Boolean(true))
            .or(just("false").to(Expr::Boolean(false)));
    
        let number = text::int::<&'a str, extra::Err<Simple<'a, char>>>(10)
            .map(|s: &str| Expr::Number(s.parse().unwrap()));
    
        let string = just::<char, &str, extra::Err<Simple<'a, char>>>('"')
            .ignore_then(
                any::<_, extra::Err<Simple<char>>>()
                    .filter(|c| *c != '"')
                    .repeated()
                    .collect::<String>()
            )
            .then_ignore(just('"'))
            .map(Expr::String);

        let left = just("left").to(Direction::Left);
        let right = just("right").to(Direction::Right);
        let forward = just("forward").to(Direction::Forward);
        let back = just("back").to(Direction::Back);
        let up = just("up").to(Direction::Up);
        let down = just("down").to(Direction::Down);
        let dir = left.or(right).or(forward).or(back).or(up).or(down).map(Expr::Direction);

        let atom = boolean.or(number).or(string).or(dir)
            .or(expr.clone().delimited_by(just('('), just(')')));

        let factor = atom.clone()
            .foldl(
                just("*").padded().to(Op::Mul)
                    .or(just("/").padded().to(Op::Div))
                    .then(atom.clone())
                    .repeated(),
                |lhs, (op, rhs)| Expr::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            );

        let term = factor.clone()
            .foldl(
                just("+").padded().to(Op::Add)
                    .or(just("-").padded().to(Op::Sub))
                    .then(factor.clone())
                    .repeated(),
                |lhs, (op, rhs)| Expr::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            );

        let comparison = term.clone()
            .foldl(
                just("==").padded().to(Op::Eq)
                    .or(just("!=").padded().to(Op::Neq))
                    .or(just("<=").padded().to(Op::Le))
                    .or(just(">=").padded().to(Op::Ge))
                    .or(just("<").padded().to(Op::Lt))
                    .or(just(">").padded().to(Op::Gt))
                    .then(term.clone())
                    .repeated(),
                |lhs, (op, rhs)| Expr::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            );

        let logic_and = comparison.clone()
            .foldl(
                just("and").padded().to(Op::And)
                .then(comparison.clone())
                .repeated(),
                |lhs,(op,rhs)| Expr::Binary{
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs)
                }
            );

        let logic_or = logic_and.clone()
            .foldl(
                just("or").padded().to(Op::Or)
                .then(logic_and.clone())
                .repeated(),
                |lhs,(op,rhs)| Expr::Binary{
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs)
                }
            );

        logic_or
    })
}

fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Simple<'a, char>>> {
    let print = just("print")
        .padded()
        .ignore_then(expr_parser())
        .map(Statement::Print);
    let _move = just("move")
        .padded()
        .ignore_then(expr_parser())
        .map(Statement::Move);
    let turn = just("turn")
        .padded()
        .ignore_then(expr_parser())
        .map(Statement::Turn);
    print.or(_move).or(turn)
}

pub fn program_parser<'a>() -> impl Parser<'a, &'a str, Vec<Statement>, extra::Err<Simple<'a, char>>> {
    statement_parser()
        .separated_by(text::newline().ignored())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}