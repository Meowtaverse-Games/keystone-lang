use keystone_lang::{Direction, Event, EventIterator, ExternalApi, eval};
use std::sync::{Arc, Mutex};

struct MyApi;
impl ExternalApi for MyApi {
    fn is_touched(&self) -> bool {
        true
    }
    fn is_empty(&self, _: Direction) -> bool {
        true
    }
}

//helper
fn next(iter: &mut EventIterator) -> Event {
    iter.next().expect("next is none").expect("error occurred")
}

#[test]
fn statement_single() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        print "Yep"
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Yep".into()));
    assert!(iter.next().is_none());
}

#[test]
fn statement_multiple() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        print "Yep"
        print "Yeah"
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Yep".into()));
    assert_eq!(next(&mut iter), Event::Print("Yeah".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_single() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        if 100 < 1000
            print "100 < 1000 is true"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("100 < 1000 is true".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_multiple() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        if "Hello" != "Happy"
            print "Hello isn't Happy"
        end
        if "Hoge" != "Hoge"
            print "and Hoge isn't Hoge"
        end
        if "Hoge" == "Hoge"
            print "but Hoge is Hoge"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Hello isn't Happy".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("but Hoge is Hoge".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_multiple_multiple() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        if true
            print "definitely works..."
            print "so there's no meaning"
        end
        if false
            print "definitely won't work..."
            print "so there's no meaning"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("definitely works...".into()));
    assert_eq!(
        next(&mut iter),
        Event::Print("so there's no meaning".into())
    );
    assert_eq!(next(&mut iter), Event::Tick);
    assert!(iter.next().is_none());
}

#[test]
fn loop_single() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        loop 3
            print "Woah"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_multiple() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        loop 3
            print "Yummy"
        end
        loop 2
            print "Delicious"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Delicious".into()));
    assert_eq!(next(&mut iter), Event::Print("Delicious".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_multiple_multiple() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        loop 2
            print "Good morning"
            print "Hello"
            print "Hi"
        end
        loop 2
            print "Good evening"
            print "Hello"
            print "Hi"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Good morning".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Print("Good morning".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("Good evening".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Print("Good evening".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert!(iter.next().is_none());
}

#[test]
fn mix() {
    let api: Arc<dyn ExternalApi + Send + Sync> = Arc::new(MyApi);
    let mut iter = eval(
        r#"
        print "Hello"
        loop 5
            print "World"
        end
        if "!"=="!"
            print "!"
        end
    "#,
        Arc::clone(&api),
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Print("!".into()));
    assert!(iter.next().is_none());
}

#[test]
fn stateful_api() {
    struct TestApi {
        touched: Mutex<bool>,
        empty: Mutex<bool>,
    }

    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            *self.touched.lock().unwrap()
        }
        fn is_empty(&self, _: Direction) -> bool {
            *self.empty.lock().unwrap()
        }
    }

    let test_api = Arc::new(TestApi {
        touched: Mutex::new(false),
        empty: Mutex::new(true),
    });

    let api_for_eval: Arc<dyn ExternalApi + Send + Sync> = test_api.clone();

    let mut iter = eval(
        r#"
        loop 5
            if not is_touched()
                move right
            end
            if is_touched()
                move up
            end
        end
    "#,
        api_for_eval,
    )
    .expect("eval failed");
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Right)); //frame 1 : not is_touched()
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Right)); //frame 2 : not is_touched()
    *test_api.touched.lock().unwrap() = true;
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Up)); //*frame 2 : is_touched()
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Up)); //frame 3 : is_touched()
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Up)); //frame 4 : is_touched()
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Tick);
    assert_eq!(next(&mut iter), Event::Move(Direction::Up)); //frame 5 : is_touched()
    assert!(iter.next().is_none());
}
