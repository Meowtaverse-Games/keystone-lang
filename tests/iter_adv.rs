use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use keystone_lang::{Direction, Event, EventIterator, ExternalApi, eval};

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

//helper
fn next(iter: &mut EventIterator) -> Event {
    iter.next().expect("next is none").expect("error occurred")
}

#[test]
fn var_use() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        msg = "Succeed!"
        print msg
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Succeed!".into()));
    assert!(iter.next().is_none());
}

#[test]
fn var_override() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        msg = "Yes"
        print msg
        msg = "Yas"
        print msg
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Yes".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Yas".into()));
    assert!(iter.next().is_none());
}

#[test]
fn var_combination() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        dad = "Father"
        print dad
        mom = "Mother"
        print mom
        print dad
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Father".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Mother".into()));
    assert_eq!(next(&mut iter), Event::Print("Father".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_loop() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        loop 3
            loop 2
                print "The day "
            end
            print "after tomorrow"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_if() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        n = 4
        if 3 < n
            if n < 5
                print "n is 4"
            end
            if n == 5
                print "n is 5"
            end
        end
        if n < 3
            if 1 < n
                print "n is 2"
            end
            if n == 1
                print "n is 1"
            end
        end
        if n == 3
            print "n is 3"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("n is 4".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert!(iter.next().is_none());
}

#[test]
fn loop_if() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        i = 2
        loop 5
            if i < 10
                print i
                print "is less than 10"
            end
            if 10 < i
                print i
                print "is greater than 10"
            end
            i = i*2
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("2".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("4".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("8".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("16".into()));
    assert_eq!(next(&mut iter), Event::Print("is greater than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("32".into()));
    assert_eq!(next(&mut iter), Event::Print("is greater than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert!(iter.next().is_none());
}

#[test]
fn different_scope() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        x = 0
        loop 5
            x = x+1
        end
        print x
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("5".into()));
    assert!(iter.next().is_none());
}

#[test]
fn too_many_loop() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        i = 0
        loop 10000
            i = i+1
            print i
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    for i in 1..=10000 {
        assert_eq!(next(&mut iter), Event::Let);
        assert_eq!(next(&mut iter), Event::Print(i.to_string()));
    }
    assert!(iter.next().is_none());
}

#[test]
fn touch_ground() {
    struct TestApi {
        touched: Mutex<bool>,
        empty: Mutex<bool>,
        signals: Mutex<HashSet<String>>,
    }

    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            *self.touched.lock().unwrap()
        }
        fn is_empty(&self, _: Direction) -> bool {
            *self.empty.lock().unwrap()
        }
        fn send_signal(&self, channel: &str) {
            if let Ok(mut g) = self.signals.lock() {
                g.insert(channel.to_owned());
            }
        }
        fn receive_signal(&self, channel: &str) -> bool {
            if let Ok(mut g) = self.signals.lock() {
                g.remove(channel)
            } else {
                false
            }
        }
    }

    let test_api = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(true),
        signals: Mutex::new(HashSet::new()),
    });

    let api_for_eval: Arc<dyn ExternalApi + Send + Sync> = test_api.clone();

    let mut iter = eval(
        r#"
        y = 0
        while not is_touched()
            move down
            y = y+1
        end
        send y
        receive y
        print y
    "#,
        api_for_eval,
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Down));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Move(Direction::Down));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Move(Direction::Down));
    assert_eq!(next(&mut iter), Event::Let);
    *test_api.touched.lock().unwrap() = true;
    assert_eq!(next(&mut iter), Event::Send("3".into()));
    assert_eq!(next(&mut iter), Event::Receive("3".into()));
    assert_eq!(next(&mut iter), Event::Print("3".into()));
    assert!(iter.next().is_none());
}

#[test]
fn complex_frame() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        x = 0
        while true
            if x < 1000
                print "hey"
                x = x + 1
            end
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Tick);
    for _ in 0..1000 {
        assert_eq!(next(&mut iter), Event::Tick);
        assert_eq!(next(&mut iter), Event::Print("hey".into()));
        assert_eq!(next(&mut iter), Event::Let);
    }
    for _ in 0..1000 {
        assert_eq!(next(&mut iter), Event::Tick);
    }
}

#[test]
fn doubled_communication() {
    struct TestApi {
        touched: Mutex<bool>,
        empty: Mutex<bool>,
        signals: Mutex<HashSet<String>>,
    }

    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            *self.touched.lock().unwrap()
        }
        fn is_empty(&self, _: Direction) -> bool {
            *self.empty.lock().unwrap()
        }
        fn send_signal(&self, channel: &str) {
            if let Ok(mut g) = self.signals.lock() {
                g.insert(channel.to_owned());
            }
        }
        fn receive_signal(&self, channel: &str) -> bool {
            if let Ok(mut g) = self.signals.lock() {
                g.remove(channel)
            } else {
                false
            }
        }
    }

    let test_api = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(true),
        signals: Mutex::new(HashSet::new()),
    });

    let api_for_eval: Arc<dyn ExternalApi + Send + Sync> = test_api.clone();

    let mut stone_a = eval(
        r#"
        send 1
        move left
        "#,
        api_for_eval.clone(),
    )
    .expect("eval a failed");
    let mut stone_b = eval(
        r#"
        receive 1
        move right
    "#,
        api_for_eval,
    )
    .expect("eval b failed");

    assert_eq!(next(&mut stone_b), Event::Wait);
    assert_eq!(next(&mut stone_b), Event::Wait);
    assert_eq!(next(&mut stone_a), Event::Send("1".to_owned()));
    assert_eq!(next(&mut stone_a), Event::Move(Direction::Left));
    assert!(stone_a.next().is_none());
    assert_eq!(next(&mut stone_b), Event::Receive("1".to_owned()));
    assert_eq!(next(&mut stone_b), Event::Move(Direction::Right));
    assert!(stone_b.next().is_none());
}

#[test]
fn complex_situation() {
    struct TestApi {
        touched: Mutex<bool>,
        empty: Mutex<bool>,
        shared_signals: Arc<Mutex<HashSet<String>>>,
    }

    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            *self.touched.lock().unwrap()
        }
        fn is_empty(&self, _: Direction) -> bool {
            *self.empty.lock().unwrap()
        }
        fn send_signal(&self, channel: &str) {
            if let Ok(mut g) = self.shared_signals.lock() {
                g.insert(channel.to_owned());
            }
        }
        fn receive_signal(&self, channel: &str) -> bool {
            if let Ok(mut g) = self.shared_signals.lock() {
                g.remove(channel)
            } else {
                false
            }
        }
    }

    let global_signals = Arc::new(Mutex::new(HashSet::new()));

    let api_a = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(false),
        shared_signals: global_signals.clone(),
    });
    let api_b = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(false),
        shared_signals: global_signals.clone(),
    });
    let api_c = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(false),
        shared_signals: global_signals.clone(),
    });

    let mut stone_a = eval(
        r#"
        dig down
        send 1
        "#,
        api_a.clone(),
    )
    .expect("eval a failed");
    let mut stone_b = eval(
        r#"
        receive 1
        while is_empty(right)
            move right
        end
        send 2
        "#,
        api_b.clone(),
    )
    .expect("eval b failed");
    let mut stone_c = eval(
        r#"
        receive 2
        move up
        "#,
        api_c.clone(),
    )
    .expect("eval c failed");

    assert_eq!(next(&mut stone_b), Event::Wait);
    assert_eq!(next(&mut stone_c), Event::Wait);
    assert_eq!(next(&mut stone_a), Event::Dig(Direction::Down));
    *api_b.empty.lock().unwrap() = true;
    assert_eq!(next(&mut stone_a), Event::Send("1".to_owned()));
    assert!(stone_a.next().is_none());
    assert_eq!(next(&mut stone_b), Event::Receive("1".to_owned()));
    assert_eq!(next(&mut stone_b), Event::Tick);
    assert_eq!(next(&mut stone_b), Event::Move(Direction::Right));
    assert_eq!(next(&mut stone_b), Event::Move(Direction::Right));
    *api_b.empty.lock().unwrap() = false;
    assert_eq!(next(&mut stone_b), Event::Send("2".to_owned()));
    assert_eq!(next(&mut stone_c), Event::Receive("2".to_owned()));
    assert_eq!(next(&mut stone_c), Event::Move(Direction::Up));
    assert!(stone_c.next().is_none());
}
