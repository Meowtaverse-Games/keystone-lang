use chumsky::{prelude::*, text::newline};
use crate::ast::*;

fn expr_parser<'a>() -> impl Parser<'a, &'a str, Expr, extra::Err<Simple<'a, char>>> {
    recursive(|expr| {
        let boolean = just("true")
            .to(Expr::Boolean(true))
            .or(just("false").to(Expr::Boolean(false)));
    
        let number = text::digits(10)
            .then(
                just('.')
                    .then(text::digits(10)) // 小数点以下の数字
                    .or_not()
            )
            .to_slice()
            .map(|s: &str| {
                if s.contains('.') {
                    Expr::Float(s.parse().unwrap())
                } else {
                    Expr::Uint(s.parse().unwrap())
                }
            });



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

        let var = text::ident::<&str, extra::Err<Simple<char>>>()
        .map(|s: &str| Expr::Var(s.to_owned()));

        let atom = boolean.or(number).or(string).or(dir)
            .or(expr.clone().delimited_by(just('('), just(')'))).or(var);

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

fn new_space<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Simple<'a, char>>> + Copy {
    any::<&'a str, extra::Err<Simple<'a, char>>>()
        .filter(|c: &char| *c == ' ' || *c == '\t')
        .ignored()
        .repeated()
}

fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Simple<'a, char>>> {
    recursive(|statement| {
        let print = just("print")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .map(Statement::Print);
        let _move = just("move")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .map(Statement::Move);
        let turn = just("turn")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .map(Statement::Turn);
        let sleep = just("sleep")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .map(Statement::Sleep);
        let _let = text::ident::<&str, extra::Err<Simple<char>>>()
            .padded_by(new_space()).then_ignore(just("=").padded_by(new_space()))
            .then(expr_parser())
            .map(|(name,expr)| Statement::Let(name.to_owned(), expr));
        let _loop = just("loop")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .then_ignore(newline())
            .then(
                statement.clone()
                    .padded_by(new_space())
                    .then_ignore(text::newline())
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>()
            )
            .then_ignore(text::newline().or_not())
            .then_ignore(new_space())
            .then_ignore(just("end").ignored())
            .map(|(count, body)| Statement::Loop(count, body));
        let _if = just("if")
            .padded_by(new_space())
            .ignore_then(expr_parser())
            .then_ignore(newline())
            .then(
                statement.clone()
                    .padded_by(new_space())
                    .then_ignore(text::newline())
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>()
            )
            .then_ignore(text::newline().or_not())
            .then_ignore(new_space())
            .then_ignore(just("end").ignored())
            .map(|(cond, body)| Statement::If(cond, body));


        print.or(_move).or(turn).or(sleep).or(_let).or(_loop).or(_if).boxed()
    })
}

fn blank_line<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Simple<'a, char>>> + Copy {
    new_space()
        .then_ignore(text::newline())
        .ignored()
}

pub fn program_parser<'a>() -> impl Parser<'a, &'a str, Vec<Statement>, extra::Err<Simple<'a, char>>> {
    let sep = blank_line().or(text::newline().ignored());

    statement_parser()
        .separated_by(sep.repeated())
        .allow_trailing()
        .collect() 
        .then_ignore(end())
}