use chumsky::prelude::*;

#[derive(Debug,Clone)]
enum Statement {
    Print(Expr),
}

#[derive(Debug,Clone)]
enum Expr {
    Number(u32),
    String(String),
    Boolean(bool),
    Binary {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>
    }
}


#[derive(Debug,Clone)]
enum Op {
    Add,Sub,Mul,Div,
    Eq,Neq,Lt,Gt,Le,Ge
}

fn expr_parser<'a>() -> impl Parser<'a, &'a str, Expr, extra::Err<Simple<'a, char>>> {
    recursive(|expr| {
        let boolean = just("true")
            .to(Expr::Boolean(true)).
            or(just("false").to(Expr::Boolean(false)));
    
        let number = text::int::<&'a str, extra::Err<Simple<'a, char>>>(10)
            .map(|s:&str| Expr::Number(s.parse().unwrap()));
    
        let string = just::<char,&str,extra::Err<Simple<'a, char>>>('"')
            .ignore_then(
                any::<_, extra::Err<Simple<char>>>()
                    .filter(|c| *c != '"')
                    .repeated()
                    .collect::<String>()
            )
            .then_ignore(just('"'))
            .map(Expr::String);

        let atom = boolean.or(number).or(string)
            .or(expr.clone().delimited_by(just('('),just(')') ));

        let op = just::<&str,&str,extra::Err<Simple<'a, char>>>("+").to(Op::Add)
            .or(just("-").to(Op::Sub))
            .or(just("*").to(Op::Mul))
            .or(just("/").to(Op::Div))
            .or(just("==").to(Op::Eq))
            .or(just("!=").to(Op::Neq))
            .or(just("<").to(Op::Lt))
            .or(just(">").to(Op::Gt))
            .or(just("<=").to(Op::Le))
            .or(just(">=").to(Op::Ge))
            .boxed();

        atom
    })
}


fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Simple<'a, char>>> {
    let print = just("print")
        .padded()
        .ignore_then(expr_parser())
        .map(Statement::Print);
    print
}

fn program_parser<'a>() -> impl Parser<'a, &'a str, Vec<Statement>, extra::Err<Simple<'a, char>>> {
        statement_parser()
        .separated_by(text::newline().ignored())
        .allow_trailing()
        .collect()
        .then_ignore(end())
}

pub fn run(input: &'static str) {
    let parser = program_parser();
    let output = parser.parse(input.trim());
    println!("{:?}", output);

    // let output = parser.parse(input.trim());
    // match output {
    //     Ok(statements) => println!("{:?}", statements),
    //     Err(errors) => {
    //         for err in errors {
    //             println!("Error: {:?}", err);
    //         }
    //     }
    // }

}