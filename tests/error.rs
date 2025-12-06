use keystone_lang::{Direction, Error, Event, Op, Side, Type, eval};

#[test]
fn unexpected_type() {
    let cases: [(&str, &str, Error); 8] = [
        ("move <String>",r#"move "Hello""#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::String }),
        ("move <Uint>",r#"move 30"#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::Uint }),
        ("move <Float>",r#"move 5.0"#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::Float }),
        ("move <Boolean>",r#"move true"#, Error::UnexpectedType { statement: String::from("Move"), found_type: Type::Boolean }),
        ("turn <String>",r#"turn "Hello""#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::String }),
        ("turn <Uint>",r#"turn 30"#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::Uint }),
        ("turn <Float>",r#"turn 5.0"#, Error::UnexpectedType { statement: String::from("Turn"), found_type: Type::Float }),
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
    let cases: [(&str, &str, Error); 8] = [
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
            if <Uint>
                [STATEMENT]
            end
        "#,r#"
            if 30
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("If"), found_type: Type::Uint }),
        (r#"
            if <Float>
                [STATEMENT]
            end
        "#,r#"
            if 5.0
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("If"), found_type: Type::Float }),
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
            loop <Float>
                [STATEMENT]
            end
        "#,r#"
            loop 5.0
                print "NG"
            end
        "#, Error::UnexpectedType { statement: String::from("Loop"), found_type: Type::Float }),
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
    let cases: [(&str, &str, Error); 10] = [
        ("<String> + <Uint>",r#"print "Hello"+100"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Uint }),
        ("<String> + <Float>",r#"print "Hello"+6.0"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Float }),
        ("<String> + <Boolean>",r#"print "Hello"+true"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Boolean }),
        ("<String> + <Direction>",r#"print "Hello"+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::String, right: Type::Direction }),
        ("<Uint> + <Boolean>",r#"print 100+true"#, Error::MismatchedTypes { op: Op::Add, left: Type::Uint, right: Type::Boolean }),
        ("<Uint> + <Direction>",r#"print 100+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::Uint, right: Type::Direction }),
        ("<Uint> + <Float>",r#"print 100+6.0"#, Error::MismatchedTypes { op: Op::Add, left: Type::Uint, right: Type::Float }),
        ("<Float> + <Boolean>",r#"print 6.0+true"#, Error::MismatchedTypes { op: Op::Add, left: Type::Float, right: Type::Boolean }),
        ("<Float> + <Direction>",r#"print 6.0+right"#, Error::MismatchedTypes { op: Op::Add, left: Type::Float, right: Type::Direction }),
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
    let cases: [(&str, &str, Error); 31] = [
        ("<String> - <String>",r#"print "Hello,"-"World!""#, Error::InvalidOperandType { op: Op::Sub, typ: Type::String }),
        ("<String> * <String>",r#"print "Hello,"*"World!""#, Error::InvalidOperandType { op: Op::Mul, typ: Type::String }),
        ("<String> / <String>",r#"print "Hello,"/"World!""#, Error::InvalidOperandType { op: Op::Div, typ: Type::String }),
        ("<String> and <String>",r#"print "Hello,"and"World!""#, Error::InvalidOperandType { op: Op::And, typ: Type::String }),
        ("<String> or <String>",r#"print "Hello,"or"World!""#, Error::InvalidOperandType { op: Op::Or, typ: Type::String }),
        ("<String> >= <String>",r#"print "Hello,">="World!""#, Error::InvalidOperandType { op: Op::Ge, typ: Type::String }),
        ("<String> <= <String>",r#"print "Hello,"<="World!""#, Error::InvalidOperandType { op: Op::Le, typ: Type::String }),
        ("<String> > <String>",r#"print "Hello,">"World!""#, Error::InvalidOperandType { op: Op::Gt, typ: Type::String }),
        ("<String> < <String>",r#"print "Hello,"<"World!""#, Error::InvalidOperandType { op: Op::Lt, typ: Type::String }),
        ("<Uint> and <Uint>",r#"print 50 and 50"#, Error::InvalidOperandType { op: Op::And, typ: Type::Uint }),
        ("<Uint> or <Uint>",r#"print 50 or 50"#, Error::InvalidOperandType { op: Op::Or, typ: Type::Uint }),
        ("<Float> and <Float>",r#"print 4.0 and 4.0"#, Error::InvalidOperandType { op: Op::And, typ: Type::Float }),
        ("<Float> or <Float>",r#"print 4.0 or 4.0"#, Error::InvalidOperandType { op: Op::Or, typ: Type::Float }),
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