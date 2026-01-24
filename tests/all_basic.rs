use keystone_lang::{eval_all,Event,Direction,Side};

#[test]
fn statement() {
    let cases: [(&str, &str, Vec<Event>); 11] = [
        ("print <String>",r#"print "Hello""#, vec![Event::Print("Hello".to_owned())]),
        ("print <Uint>",r#"print 30"#, vec![Event::Print("30".to_owned())]),
        ("print <Float>",r#"print 3.5"#, vec![Event::Print("3.5".to_owned())]),
        ("print <Boolean>",r#"print true"#, vec![Event::Print("true".to_owned())]),
        ("print <Direction>",r#"print up"#, vec![Event::Print("Up".to_owned())]),
        ("print <Side>",r#"print right"#, vec![Event::Print("Right".to_owned())]),
        ("move <Direction>",r#"move forward"#, vec![Event::Move(Direction::Forward)]),
        ("turn <Side>",r#"turn left"#, vec![Event::Turn(Side::Left)]),
        ("dig <Direction>",r#"dig back"#, vec![Event::Dig(Direction::Back)]),
        ("sleep <Float>",r#"sleep 1.2"#, vec![Event::Sleep(1.2)]),
        ("<Var> = <Expr>",r#"name = "Taro""#, vec![Event::Let]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn super_statement() {
    let cases: [(&str, &str, Vec<Event>); 4] = [
        (r#"
            if true
                [STATEMENT]
            end
        "#,r#"
            if true
                print "Ok"
            end
        "#, vec![Event::Print("Ok".to_owned())]),
        (r#"
            if false
                [STATEMENT]
            end
        "#,r#"
            if false
                print "NG"
            end
        "#,Vec::<Event>::new()),
        (r#"
            loop <Number>
                [STATEMENT]
            end
        "#,r#"
            loop 3
                print "Ok"
            end
        "#, vec![Event::Print("Ok".to_owned());3]),
        (r#"
            while <Boolean>
                [STATEMENT]
            end
        "#,r#"
            x = 0
            while x < 3
                print "White"
                x = x + 1
            end
        "#, vec![
            Event::Let,
            Event::Print("White".to_owned()),
            Event::Let,
            Event::Print("White".to_owned()),
            Event::Let,
            Event::Print("White".to_owned()),
            Event::Let,
        ]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn uint() {
    let cases: [(&str, &str, Vec<Event>); 10] = [
        ("<Uint> + <Uint>",r#"print 60+4"#, vec![Event::Print("64".to_owned())]),
        ("<Uint> - <Uint>",r#"print 60-4"#, vec![Event::Print("56".to_owned())]),
        ("<Uint> * <Uint>",r#"print 60*4"#, vec![Event::Print("240".to_owned())]),
        ("<Uint> / <Uint>",r#"print 60/4"#, vec![Event::Print("15".to_owned())]),
        ("<Uint> == <Uint>",r#"print 60==40"#, vec![Event::Print("false".to_owned())]),
        ("<Uint> != <Uint>",r#"print 60!=40"#, vec![Event::Print("true".to_owned())]),
        ("<Uint> >= <Uint>",r#"print 60>=40"#, vec![Event::Print("true".to_owned())]),
        ("<Uint> <= <Uint>",r#"print 60<=40"#, vec![Event::Print("false".to_owned())]),
        ("<Uint> > <Uint>",r#"print 60>40"#, vec![Event::Print("true".to_owned())]),
        ("<Uint> < <Uint>",r#"print 60<40"#, vec![Event::Print("false".to_owned())]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn float() {
    let cases: [(&str, &str, Vec<Event>); 10] = [
        ("<Float> + <Float>",r#"print 3.9+1.2"#, vec![Event::Print("5.1".to_owned())]),
        ("<Float> - <Float>",r#"print 3.9-1.2"#, vec![Event::Print("2.7".to_owned())]),
        ("<Float> * <Float>",r#"print 3.9*1.2"#, vec![Event::Print("4.68".to_owned())]),
        ("<Float> / <Float>",r#"print 3.9/1.2"#, vec![Event::Print("3.25".to_owned())]),
        ("<Float> == <Float>",r#"print 3.9==1.2"#, vec![Event::Print("false".to_owned())]),
        ("<Float> != <Float>",r#"print 3.9!=1.2"#, vec![Event::Print("true".to_owned())]),
        ("<Float> >= <Float>",r#"print 3.9>=1.2"#, vec![Event::Print("true".to_owned())]),
        ("<Float> <= <Float>",r#"print 3.9<=1.2"#, vec![Event::Print("false".to_owned())]),
        ("<Float> > <Float>",r#"print 3.9>1.2"#, vec![Event::Print("true".to_owned())]),
        ("<Float> < <Float>",r#"print 3.9<1.2"#, vec![Event::Print("false".to_owned())]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn string() {
    let cases: [(&str, &str, Vec<Event>); 3] = [
        ("<String> + <String>",r#"print "hello,"+"world!""#, vec![Event::Print("hello,world!".to_owned())]),
        ("<String> == <String>",r#"print "hello,"=="world!""#, vec![Event::Print("false".to_owned())]),
        ("<String> != <String>",r#"print "hello,"!="world!""#, vec![Event::Print("true".to_owned())]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}


#[test]
fn boolean() {
    let cases: [(&str, &str, Vec<Event>); 5] = [
        ("<Boolean> and <Boolean>",r#"print true and false"#, vec![Event::Print("false".to_owned())]),
        ("<Boolean> or <Boolean>",r#"print true or false"#, vec![Event::Print("true".to_owned())]),
        ("not <Boolean>",r#"print not false"#, vec![Event::Print("true".to_owned())]),
        ("<Boolean> == <Boolean>",r#"print true == false"#, vec![Event::Print("false".to_owned())]),
        ("<Boolean> != <Boolean>",r#"print true != false"#, vec![Event::Print("true".to_owned())]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}