use keystone_lang::{eval,Event,EventIterator,ExternalApi};

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
fn var_use(){
    let mut iter = eval(r#"
        msg = "Succeed!"
        print msg
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Succeed!".into()));
    assert!(iter.next().is_none());
}

#[test]
fn var_override(){
    let mut iter = eval(r#"
        msg = "Yes"
        print msg
        msg = "Yas"
        print msg
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Yes".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Yas".into()));
    assert!(iter.next().is_none());
}

#[test]
fn var_combination(){
    let mut iter = eval(r#"
        dad = "Father"
        print dad
        mom = "Mother"
        print mom
        print dad
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Father".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("Mother".into()));
    assert_eq!(next(&mut iter), Event::Print("Father".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_loop(){
    let mut iter = eval(r#"
        loop 3
            loop 2
                print "The day "
            end
            print "after tomorrow"
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("The day ".into()));
    assert_eq!(next(&mut iter), Event::Print("after tomorrow".into()));
    assert!(iter.next().is_none());
}

#[test]
fn if_if(){
    let mut iter = eval(r#"
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
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("n is 4".into()));
    assert!(iter.next().is_none());
}

#[test]
fn loop_if(){
    let mut iter = eval(r#"
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
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("2".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("4".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("8".into()));
    assert_eq!(next(&mut iter), Event::Print("is less than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("16".into()));
    assert_eq!(next(&mut iter), Event::Print("is greater than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("32".into()));
    assert_eq!(next(&mut iter), Event::Print("is greater than 10".into()));
    assert_eq!(next(&mut iter), Event::Let);
    assert!(iter.next().is_none());
}

#[test]
fn different_scope(){
    let mut iter = eval(r#"
        x = 0
        loop 5
            x = x+1
        end
        print x
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Let);
    assert_eq!(next(&mut iter), Event::Print("5".into()));
    assert!(iter.next().is_none());
}

#[test]
fn too_many_loop(){
    let mut iter = eval(r#"
        i = 0
        loop 10000
            i = i+1
            print i
        end
    "#,&API).expect("eval failed");
    assert_eq!(next(&mut iter), Event::Let);
    for i in 1..=10000 {
        assert_eq!(next(&mut iter), Event::Let);
        assert_eq!(next(&mut iter), Event::Print(i.to_string()));
    }
    assert!(iter.next().is_none());
}