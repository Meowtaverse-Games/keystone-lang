use keystone_lang::{Direction, Error, Event, Op, Side, Type, eval};

#[test]
fn unexpected_type() {
    let cases: [(&str, &str, Error); 6] = [
        ("move <String>",r#"move "Hello""#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::String }),
        ("move <Number>",r#"move 30"#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::Number }),
        ("move <Boolean>",r#"move true"#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::Boolean }),
        ("turn <String>",r#"turn "Hello""#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::String }),
        ("turn <Number>",r#"turn 30"#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::Number }),
        ("turn <Boolean>",r#"turn true"#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::Boolean }),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}

#[test]
fn super_unexpected_type() {
    let cases: [(&str, &str, Error); 6] = [
        (r#"
            if <String>
                [STATEMENT]
            end
        "#,r#"
            if "Hello"
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("If"), found_type: Type::String }),
        (r#"
            if <Number>
                [STATEMENT]
            end
        "#,r#"
            if 30
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("If"), found_type: Type::Number }),
        (r#"
            if <Direction>
                [STATEMENT]
            end
        "#,r#"
            if down
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("If"), found_type: Type::Direction }),
        (r#"
            loop <String>
                [STATEMENT]
            end
        "#,r#"
            loop "Hello"
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("Loop"), found_type: Type::String }),
        (r#"
            loop <Boolean>
                [STATEMENT]
            end
        "#,r#"
            loop true
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("Loop"), found_type: Type::Boolean }),
        (r#"
            loop <Direction>
                [STATEMENT]
            end
        "#,r#"
            loop left
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("Loop"), found_type: Type::Direction }),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}

#[test]
fn mismatched_types() {
    let cases: [(&str, &str, Error); 6] = [
        ("<String> + <Number>",r#"print "Hello"+100"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Number }),
        ("<String> + <Boolean>",r#"print "Hello"+true"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Boolean }),
        ("<String> + <Direction>",r#"print "Hello"+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Direction }),
        ("<Number> + <Boolean>",r#"print 100+true"#, Error::MismatchedTypes { op: Op::Add, left: Type::Number, right: Type::Boolean }),
        ("<Number> + <Direction>",r#"print 100+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::Number, right: Type::Direction }),
        ("<Boolean> + <Direction>",r#"print true+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::Boolean, right: Type::Direction }),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}

#[test]
fn invalid_operand_type() {
    let cases: [(&str, &str, Error); 29] = [
        ("<String> - <String>",r#"print "Hello,"-"World!""#, Error::InvalidOperandType { op: Op::Sub, typ: Type::String }),
        ("<String> * <String>",r#"print "Hello,"*"World!""#, Error::InvalidOperandType { op: Op::Mul, typ: Type::String }),
        ("<String> / <String>",r#"print "Hello,"/"World!""#, Error::InvalidOperandType { op: Op::Div, typ: Type::String }),
        ("<String> and <String>",r#"print "Hello,"and"World!""#, Error::InvalidOperandType { op: Op::And, typ: Type::String }),
        ("<String> or <String>",r#"print "Hello,"or"World!""#, Error::InvalidOperandType { op: Op::Or, typ: Type::String }),
        ("<String> >= <String>",r#"print "Hello,">="World!""#, Error::InvalidOperandType { op: Op::Ge, typ: Type::String }),
        ("<String> <= <String>",r#"print "Hello,"<="World!""#, Error::InvalidOperandType { op: Op::Le, typ: Type::String }),
        ("<String> > <String>",r#"print "Hello,">"World!""#, Error::InvalidOperandType { op: Op::Gt, typ: Type::String }),
        ("<String> < <String>",r#"print "Hello,"<"World!""#, Error::InvalidOperandType { op: Op::Lt, typ: Type::String }),
        ("<Number> and <Number>",r#"print 50 and 50"#, Error::InvalidOperandType { op: Op::And, typ: Type::Number }),
        ("<Number> or <Number>",r#"print 50 or 50"#, Error::InvalidOperandType { op: Op::Or, typ: Type::Number }),
        ("<Boolean> + <Boolean>",r#"print true+true"#, Error::InvalidOperandType { op: Op::Add, typ: Type::Boolean }),
        ("<Boolean> - <Boolean>",r#"print true-true"#, Error::InvalidOperandType { op: Op::Sub, typ: Type::Boolean }),
        ("<Boolean> * <Boolean>",r#"print true*true"#, Error::InvalidOperandType { op: Op::Mul, typ: Type::Boolean }),
        ("<Boolean> / <Boolean>",r#"print true/true"#, Error::InvalidOperandType { op: Op::Div, typ: Type::Boolean }),
        ("<Boolean> >= <Boolean>",r#"print true>=true"#, Error::InvalidOperandType { op: Op::Ge, typ: Type::Boolean }),
        ("<Boolean> <= <Boolean>",r#"print true<=true"#, Error::InvalidOperandType { op: Op::Le, typ: Type::Boolean }),
        ("<Boolean> > <Boolean>",r#"print true>true"#, Error::InvalidOperandType { op: Op::Gt, typ: Type::Boolean }),
        ("<Boolean> < <Boolean>",r#"print true<true"#, Error::InvalidOperandType { op: Op::Lt, typ: Type::Boolean }),
        ("<Direction> + <Direction>",r#"print forward+back"#, Error::InvalidOperandType { op: Op::Add, typ: Type::Direction }),
        ("<Direction> - <Direction>",r#"print forward-back"#, Error::InvalidOperandType { op: Op::Sub, typ: Type::Direction }),
        ("<Direction> * <Direction>",r#"print forward*back"#, Error::InvalidOperandType { op: Op::Mul, typ: Type::Direction }),
        ("<Direction> / <Direction>",r#"print forward/back"#, Error::InvalidOperandType { op: Op::Div, typ: Type::Direction }),
        ("<Direction> >= <Direction>",r#"print forward>=back"#, Error::InvalidOperandType { op: Op::Ge, typ: Type::Direction }),
        ("<Direction> <= <Direction>",r#"print forward<=back"#, Error::InvalidOperandType { op: Op::Le, typ: Type::Direction }),
        ("<Direction> > <Direction>",r#"print forward>back"#, Error::InvalidOperandType { op: Op::Gt, typ: Type::Direction }),
        ("<Direction> < <Direction>",r#"print forward<back"#, Error::InvalidOperandType { op: Op::Lt, typ: Type::Direction }),
        ("<Direction> and <Direction>",r#"print forward and back"#, Error::InvalidOperandType { op: Op::And, typ: Type::Direction }),
        ("<Direction> or <Direction>",r#"print forward or back"#, Error::InvalidOperandType { op: Op::Or, typ: Type::Direction }),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}


#[test]
fn name_error() {
    let cases: [(&str, &str, Error); 2] = [
        ("UNDEFINED",r#"print name"#, Error::NameError { name: String::from("name") }),
        ("UNDEFINED YET",r#"
            print name
            name = "Taro"
        "#, Error::NameError { name: String::from("name") }),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}

#[test]
fn zero_division_error() {
    let cases: [(&str, &str, Error); 2] = [
        ("<Number> / 0",r#"print 10/0"#, Error::ZeroDivisionError),
        ("<Number> / <Var(0)>",r#"
            p = 0
            print 5/p
        "#, Error::ZeroDivisionError),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}

#[test]
fn too_large_number() {
    let cases: [(&str, &str, Error); 1] = [
        ("GENERATED TOO LARGE",r#"
            n = 2
            loop 6
                n = n*n
                print n
            end
        "#, Error::TooLargeNumber),
    ];

    for (case, src, expected) in cases {
        if let Err(e) = eval(src){
            assert_eq!(e, expected, "{case}");
        }
    }
}