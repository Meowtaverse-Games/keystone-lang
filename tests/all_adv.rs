use std::sync::Arc;

use keystone_lang::{Direction, Event, ExternalApi, eval_all};

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
fn var_use() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        n = 30
        print n
    "#,
            api
        )
        .expect("eval failed"),
        vec![Event::Let, Event::Print("30".to_owned())]
    );
}

#[test]
fn var_combination() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        x = 3 + 5
        print x
        y = x * 2
        print y
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Print("8".to_owned()),
            Event::Let,
            Event::Print("16".to_owned())
        ]
    );
}

#[test]
fn float_var() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        x = 1.8
        print x
        y = x * 2.0
        print y
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Print("1.8".to_owned()),
            Event::Let,
            Event::Print("3.6".to_owned())
        ]
    );
}

#[test]
fn loop_statements() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        loop 5
            print "Hello"
            print 5*2
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Tick,
            Event::Print("Hello".to_owned()),
            Event::Print("10".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("10".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("10".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("10".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("10".to_owned()),
        ]
    );
}

#[test]
fn if_var_combination() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        x = 10
        if x > 5
            print "Yes"
        end
        if x <= 5
            print "No"
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Print("Yes".to_owned()),
            Event::Tick
        ]
    );
}

#[test]
fn loop_loop() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    assert_eq!(
        eval_all(
            r#"
        loop 2
            loop 3
                print "Hello"
            end
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Tick,
            Event::Tick,
            Event::Print("Hello".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Tick,
            Event::Print("Hello".to_owned()),
            Event::Print("Hello".to_owned()),
            Event::Print("Hello".to_owned()),
        ]
    );
}

#[test]
fn while_while() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        p = 0
        q = 0
        while p < 3
            q = 0
            while q < 2
                print p+q
                q = q + 1
            end
            p = p+1
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Print("0".to_owned()),
            Event::Let,
            Event::Print("1".to_owned()),
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Print("1".to_owned()),
            Event::Let,
            Event::Print("2".to_owned()),
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Print("2".to_owned()),
            Event::Let,
            Event::Print("3".to_owned()),
            Event::Let,
            Event::Let,
        ]
    );
}

#[test]
fn if_if() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        x = 30
        if x > 20
            if x < 40
                print "Medium"
            end
            if 40 < x
                print "Large"
            end
        end
        if x < 20
            print "Small"
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Tick,
            Event::Print("Medium".to_owned()),
            Event::Tick,
            Event::Tick
        ]
    );
}

#[test]
fn not_not() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        x = true
        if not not x
            print "na na..."
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![Event::Let, Event::Tick, Event::Print("na na...".to_owned())]
    );
}

#[test]
fn not_toggle() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        z = true
        loop 3
            z = not z
            if not z
                print "Switching..."
            end
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Print("Switching...".to_owned()),
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Print("Switching...".to_owned())
        ]
    );
}

#[test]
fn not_bin() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        if not (3 > 5)
            print "3 is not greater than 5."
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Tick,
            Event::Print("3 is not greater than 5.".to_owned())
        ]
    );
}

#[test]
fn loop_if() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        n = 2
        loop 4
            n = n*n
            if 10000 < n
                print n
            end
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Tick,
            Event::Print("65536".to_owned())
        ]
    );
}

#[test]
fn complex_binary() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        n = (3+5)*8/2+50
        m = (n+8)/(15-5)+(5+1)*(2+3)/10-2
        if m*8 < n
            print n-m*8
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Print("2".to_owned())
        ]
    );
}

#[test]
fn released_scope() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        x = 0
        loop 3
            x = x + 1
        end
        print x
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Print("3".to_owned())
        ]
    );
}

#[test]
fn switch_direction() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        d = forward
        s = true
        loop 4
            if d == forward and s
                d = right
                s = false
            end
            if d == right and s
                d = back
                s = false
            end
            if d == back and s
                d = left
                s = false
            end
            if d == left and s
                d = forward
                s = false
            end
            s = true
            move d
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Move(Direction::Right),
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Move(Direction::Back),
            Event::Tick,
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Move(Direction::Left),
            Event::Tick,
            Event::Tick,
            Event::Tick,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Move(Direction::Forward),
        ]
    );
}

#[test]
fn sleep_timer_step() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        d = 0.0
        loop 5
            d = d+0.5
            sleep d
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Sleep(0.5),
            Event::Let,
            Event::Sleep(1.0),
            Event::Let,
            Event::Sleep(1.5),
            Event::Let,
            Event::Sleep(2.0),
            Event::Let,
            Event::Sleep(2.5),
        ]
    );
}

#[test]
fn complex_while() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);

    assert_eq!(
        eval_all(
            r#"
        x = 0
        y = 0
        while x < 5
            x = x + 1
        end
        while y < 3
            y = y + 1
        end
        print x+y
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Tick,
            Event::Let,
            Event::Let,
            Event::Let,
            Event::Print("8".to_owned())
        ]
    );
}

#[test]
fn complex_api() {
    struct TestApi;
    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            false
        }
        fn is_empty(&self, _: Direction) -> bool {
            true
        }
        fn send_signal(&self, _channel: &str) {}
        fn receive_signal(&self, _channel: &str) -> bool {
            true
        }
    }
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(TestApi);

    assert_eq!(
        eval_all(
            r#"
        x = is_touched()
        y = is_empty(right)
        receive 100
        if x == y
            print "x == y"
        end
        if not x == y
            print "x != y"
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Let,
            Event::Receive("100".to_owned()),
            Event::Tick,
            Event::Tick,
            Event::Print("x != y".to_owned())
        ]
    );
}

#[test]
fn dynamic_api() {
    struct TestApi;
    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            false
        }
        fn is_empty(&self, _: Direction) -> bool {
            true
        }
        fn send_signal(&self, _channel: &str) {}
        fn receive_signal(&self, _channel: &str) -> bool {
            true
        }
    }
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(TestApi);

    assert_eq!(
        eval_all(
            r#"
        n = 5
        receive n
        if not is_touched()
            print "false"
        end
    "#,
            api
        )
        .expect("eval failed"),
        vec![
            Event::Let,
            Event::Receive("5".to_owned()),
            Event::Tick,
            Event::Print("false".to_owned())
        ]
    );
}
