use core::panic;

use keystone_lang::{eval,Event,EventIterator,Error,ExternalApi};

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

//helper(weak)
fn next(iter: &mut EventIterator) -> Result<Event,Error> {
    iter.next().expect("next is none")
}

#[test]
fn error_lazy(){
    let mut iter = eval(r#"
        x = 10
        y = 0
        print x / y
        print "Hey"
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Err(Error::ZeroDivisionError));
    assert_eq!(next(&mut iter), Ok(Event::Print("Hey".into())));
    assert!(iter.next().is_none());
}

#[test]
fn error_pre(){
    let iter = eval(r#"
        x = 10
        print y
        print x
    "#,&API);
    if let Err(err) = iter{
        assert_eq!(err, Error::NameError { name: "y".into() });
    }else{
        panic!("No Error Occured")
    }
}

#[test]
fn later_loop(){
    let mut iter = eval(r#"
        x = 3
        loop 3
            x = x-1
            print 10 / x
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Ok(Event::Print("5".into())));
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Ok(Event::Print("10".into())));
    assert_eq!(next(&mut iter), Ok(Event::Let));
    assert_eq!(next(&mut iter), Err(Error::ZeroDivisionError));
    assert!(iter.next().is_none());
}