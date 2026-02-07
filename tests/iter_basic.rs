use keystone_lang::{Direction, Event, EventIterator, ExternalApi, eval};
use std::cell::RefCell;

struct MyApi;
impl ExternalApi for MyApi {
    fn is_touched(&self) -> bool {
        true
    }
    fn is_empty(&self) -> bool {
        true
    }
}
const API:MyApi = MyApi;

//helper
fn next(iter: &mut EventIterator) -> Event {
    iter.next().expect("next is none").expect("error occurred")
}

#[test]
fn statement_single(){
    let mut iter = eval(r#"
        print "Yep"
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Yep".into()));
    assert!(iter.next().is_none());
}

#[test]
fn statement_multiple(){
    let mut iter = eval(r#"
        print "Yep"
        print "Yeah"
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Yep".into()));
    assert_eq!(next(&mut iter), Event::Print("Yeah".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_single(){
    let mut iter = eval(r#"
        if 100 < 1000
            print "100 < 1000 is true"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("100 < 1000 is true".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_multiple(){
    let mut iter = eval(r#"
        if "Hello" != "Happy"
            print "Hello isn't Happy"
        end
        if "Hoge" != "Hoge"
            print "and Hoge isn't Hoge"
        end
        if "Hoge" == "Hoge"
            print "but Hoge is Hoge"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Hello isn't Happy".into()));
    assert_eq!(next(&mut iter), Event::Print("but Hoge is Hoge".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_multiple_multiple(){
    let mut iter = eval(r#"
        if true
            print "definitely works..."
            print "so there's no meaning"
        end
        if false
            print "definitely won't work..."
            print "so there's no meaning"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("definitely works...".into()));
    assert_eq!(next(&mut iter), Event::Print("so there's no meaning".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_single(){
    let mut iter = eval(r#"
        loop 3
            print "Woah"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert_eq!(next(&mut iter), Event::Print("Woah".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_multiple(){
    let mut iter = eval(r#"
        loop 3
            print "Yummy"
        end
        loop 2
            print "Delicious"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Print("Yummy".into()));
    assert_eq!(next(&mut iter), Event::Print("Delicious".into()));
    assert_eq!(next(&mut iter), Event::Print("Delicious".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_multiple_multiple(){
    let mut iter = eval(r#"
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
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Good morning".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Print("Good morning".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Print("Good evening".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert_eq!(next(&mut iter), Event::Print("Good evening".into()));
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("Hi".into()));
    assert!(iter.next().is_none());
}

#[test]
fn mix(){
    let mut iter = eval(r#"
        print "Hello"
        loop 5
            print "World"
        end
        if "!"=="!"
            print "!"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("Hello".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("World".into()));
    assert_eq!(next(&mut iter), Event::Print("!".into()));
    assert!(iter.next().is_none());
}

#[test]
fn stateful_api(){
    struct TestApi {
        touched: RefCell<bool>,
        empty: RefCell<bool>,
    }

    impl ExternalApi for TestApi {
        fn is_touched(&self) -> bool {
            *self.touched.borrow()
        }
        fn is_empty(&self) -> bool {
            *self.empty.borrow()
        }
    }

    let api = TestApi {
        touched: RefCell::new(false),
        empty: RefCell::new(true),
    };

    let mut iter = eval(r#"
        loop 5
            if not is_touched()
                move right
            end
            if is_touched()
                move up
            end
        end
    "#,&api).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Move(Direction::Right));
    assert_eq!(next(&mut iter), Event::Move(Direction::Right));
    *api.touched.borrow_mut() = true;
    assert_eq!(next(&mut iter), Event::Move(Direction::Up));
    assert_eq!(next(&mut iter), Event::Move(Direction::Up));
    assert_eq!(next(&mut iter), Event::Move(Direction::Up));
    assert_eq!(next(&mut iter), Event::Move(Direction::Up)); //?
    assert!(iter.next().is_none());
}