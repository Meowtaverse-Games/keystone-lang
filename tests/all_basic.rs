use std::sync::Arc;

use keystone_lang::{Direction, Event, ExternalApi, Side, eval_all};

struct MyApi;
impl ExternalApi for MyApi {
    fn is_touched(&self) -> bool {
        true
    }
    fn is_empty(&self, _: Direction) -> bool {
        true
    }
    fn send_signal(&self, _channel: &str) {}
    fn receive_signal(&self, _channel: &str) -> bool {
        true
    }
}

#[test]
fn statement() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 11] = [
        (
            "print <String>",
            r#"print "Hello""#,
            vec![Event::Print("Hello".to_owned())],
        ),
        (
            "print <Uint>",
            r#"print 30"#,
            vec![Event::Print("30".to_owned())],
        ),
        (
            "print <Float>",
            r#"print 3.5"#,
            vec![Event::Print("3.5".to_owned())],
        ),
        (
            "print <Boolean>",
            r#"print true"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "print <Direction>",
            r#"print up"#,
            vec![Event::Print("Up".to_owned())],
        ),
        (
            "print <Side>",
            r#"print right"#,
            vec![Event::Print("Right".to_owned())],
        ),
        (
            "move <Direction>",
            r#"move forward"#,
            vec![Event::Move(Direction::Forward)],
        ),
        ("turn <Side>", r#"turn left"#, vec![Event::Turn(Side::Left)]),
        (
            "dig <Direction>",
            r#"dig back"#,
            vec![Event::Dig(Direction::Back)],
        ),
        ("sleep <Float>", r#"sleep 1.2"#, vec![Event::Sleep(1.2)]),
        ("<Var> = <Expr>", r#"name = "Taro""#, vec![Event::Let]),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn place_directions() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let result = eval_all(
        r#"
        place up
        place down
        place left
        place right
        "#,
        api,
    )
    .expect("eval failed");

    assert_eq!(
        result,
        vec![
            Event::Place(Direction::Up),
            Event::Place(Direction::Down),
            Event::Place(Direction::Left),
            Event::Place(Direction::Right),
        ]
    );
}

#[test]
fn super_statement() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 4] = [
        (
            r#"
            if true
                [STATEMENT]
            end
        "#,
            r#"
            if true
                print "Ok"
            end
        "#,
            vec![Event::Tick, Event::Print("Ok".to_owned())],
        ),
        (
            r#"
            if false
                [STATEMENT]
            end
        "#,
            r#"
            if false
                print "NG"
            end
        "#,
            vec![Event::Tick],
        ),
        (
            r#"
            loop <Number>
                [STATEMENT]
            end
        "#,
            r#"
            loop 3
                print "Ok"
            end
        "#,
            vec![
                Event::Tick,
                Event::Print("Ok".to_owned()),
                Event::Print("Ok".to_owned()),
                Event::Print("Ok".to_owned()),
            ],
        ),
        (
            r#"
            while <Boolean>
                [STATEMENT]
            end
        "#,
            r#"
            x = 0
            while x < 3
                print "White"
                x = x + 1
            end
        "#,
            vec![
                Event::Let,
                Event::Tick,
                Event::Print("White".to_owned()),
                Event::Let,
                Event::Print("White".to_owned()),
                Event::Let,
                Event::Print("White".to_owned()),
                Event::Let,
            ],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn uint() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 10] = [
        (
            "<Uint> + <Uint>",
            r#"print 60+4"#,
            vec![Event::Print("64".to_owned())],
        ),
        (
            "<Uint> - <Uint>",
            r#"print 60-4"#,
            vec![Event::Print("56".to_owned())],
        ),
        (
            "<Uint> * <Uint>",
            r#"print 60*4"#,
            vec![Event::Print("240".to_owned())],
        ),
        (
            "<Uint> / <Uint>",
            r#"print 60/4"#,
            vec![Event::Print("15".to_owned())],
        ),
        (
            "<Uint> == <Uint>",
            r#"print 60==40"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Uint> != <Uint>",
            r#"print 60!=40"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Uint> >= <Uint>",
            r#"print 60>=40"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Uint> <= <Uint>",
            r#"print 60<=40"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Uint> > <Uint>",
            r#"print 60>40"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Uint> < <Uint>",
            r#"print 60<40"#,
            vec![Event::Print("false".to_owned())],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn float() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 10] = [
        (
            "<Float> + <Float>",
            r#"print 3.9+1.2"#,
            vec![Event::Print("5.1".to_owned())],
        ),
        (
            "<Float> - <Float>",
            r#"print 3.9-1.2"#,
            vec![Event::Print("2.7".to_owned())],
        ),
        (
            "<Float> * <Float>",
            r#"print 3.9*1.2"#,
            vec![Event::Print("4.68".to_owned())],
        ),
        (
            "<Float> / <Float>",
            r#"print 3.9/1.2"#,
            vec![Event::Print("3.25".to_owned())],
        ),
        (
            "<Float> == <Float>",
            r#"print 3.9==1.2"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Float> != <Float>",
            r#"print 3.9!=1.2"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Float> >= <Float>",
            r#"print 3.9>=1.2"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Float> <= <Float>",
            r#"print 3.9<=1.2"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Float> > <Float>",
            r#"print 3.9>1.2"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Float> < <Float>",
            r#"print 3.9<1.2"#,
            vec![Event::Print("false".to_owned())],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn string() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 3] = [
        (
            "<String> + <String>",
            r#"print "hello,"+"world!""#,
            vec![Event::Print("hello,world!".to_owned())],
        ),
        (
            "<String> == <String>",
            r#"print "hello,"=="world!""#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<String> != <String>",
            r#"print "hello,"!="world!""#,
            vec![Event::Print("true".to_owned())],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn boolean() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let cases: [(&str, &str, Vec<Event>); 5] = [
        (
            "<Boolean> and <Boolean>",
            r#"print true and false"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Boolean> or <Boolean>",
            r#"print true or false"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "not <Boolean>",
            r#"print not false"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "<Boolean> == <Boolean>",
            r#"print true == false"#,
            vec![Event::Print("false".to_owned())],
        ),
        (
            "<Boolean> != <Boolean>",
            r#"print true != false"#,
            vec![Event::Print("true".to_owned())],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}

#[test]
fn api() {
    struct TestApi;
    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            true
        }
        fn is_empty(&self, _: Direction) -> bool {
            false
        }
        fn send_signal(&self, _channel: &str) {}
        fn receive_signal(&self, _channel: &str) -> bool {
            true
        }
    }
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(TestApi);

    let cases: [(&str, &str, Vec<Event>); 6] = [
        (
            "is_touched()",
            r#"print is_touched()"#,
            vec![Event::Print("true".to_owned())],
        ),
        (
            "is_empty(<Direction>)",
            r#"print is_empty(right)"#,
            vec![Event::Print("false".to_owned())],
        ),
        // randはadvにてテスト
        (
            "send <Uint>",
            r#"send 1"#,
            vec![Event::Send("1".to_owned())],
        ),
        (
            "send <String>",
            r#"send "hi""#,
            vec![Event::Send("hi".to_owned())],
        ),
        (
            "receive <Uint>",
            r#"receive 1"#,
            vec![Event::Receive("1".to_owned())],
        ),
        (
            "receive <String>",
            r#"receive "hi""#,
            vec![Event::Receive("hi".to_owned())],
        ),
    ];

    for (case, src, expected) in cases {
        let result = eval_all(src, Arc::clone(&api)).expect("eval failed");
        assert_eq!(result, expected, "{case}");
    }
}
