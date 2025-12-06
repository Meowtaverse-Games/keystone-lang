use keystone_lang::{eval,Event,Direction,Side};

#[test]
fn var_use(){
    assert_eq!(eval(r#"
        n = 30
        print n
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Print("30".to_owned())
    ]);
}

#[test]
fn var_combination(){
    assert_eq!(eval(r#"
        x = 3 + 5
        print x
        y = x * 2
        print y
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Print("8".to_owned()),
        Event::Let,
        Event::Print("16".to_owned())
    ]);
}

#[test]
fn float_var(){
    assert_eq!(eval(r#"
        x = 1.8
        print x
        y = x * 2.0
        print y
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Print("1.8".to_owned()),
        Event::Let,
        Event::Print("3.6".to_owned())
    ]);
}

#[test]
fn loop_statements(){
    assert_eq!(eval(r#"
        loop 5
            print "Hello"
            print 5*2
        end
    "#).expect("eval failed"),vec![
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
    ]);
}

#[test]
fn if_var_combination(){
    assert_eq!(eval(r#"
        x = 10
        if x > 5
            print "Yes"
        end
        if x <= 5
            print "No"
        end
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Print("Yes".to_owned())
    ]);
}

#[test]
fn loop_loop(){
    assert_eq!(eval(r#"
        loop 2
            loop 3
                print "Hello"
            end
        end
    "#).expect("eval failed"),vec![
        Event::Print("Hello".to_owned()),
        Event::Print("Hello".to_owned()),
        Event::Print("Hello".to_owned()),
        Event::Print("Hello".to_owned()),
        Event::Print("Hello".to_owned()),
        Event::Print("Hello".to_owned()),
    ]);
}


#[test]
fn if_if(){
    assert_eq!(eval(r#"
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
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Print("Medium".to_owned())
    ]);
}


#[test]
fn loop_if(){
    assert_eq!(eval(r#"
        n = 2
        loop 4
            n = n*n
            if 10000 < n
                print n
            end
        end
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Print("65536".to_owned())
    ]);
}

#[test]
fn complex_binary(){
    assert_eq!(eval(r#"
        n = (3+5)*8/2+50
        m = (n+8)/(15-5)+(5+1)*(2+3)/10-2
        if m*8 < n
            print n-m*8
        end
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Let,
        Event::Print("2".to_owned())
    ]);
}

#[test]
fn released_scope(){
    assert_eq!(eval(r#"
        x = 0
        loop 3
            x = x + 1
        end
        print x
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Print("3".to_owned())
    ]);
}

#[test]
fn switch_direction(){
    assert_eq!(eval(r#"
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
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Move(Direction::Right),
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Move(Direction::Back),
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Move(Direction::Left),
        Event::Let,
        Event::Let,
        Event::Let,
        Event::Move(Direction::Forward),
    ]);
}



#[test]
fn sleep_timer_step(){
    assert_eq!(eval(r#"
        d = 0.0
        loop 5
            d = d+0.5
            sleep d
        end
    "#).expect("eval failed"),vec![
        Event::Let,
        Event::Let,Event::Sleep(0.5),
        Event::Let,Event::Sleep(1.0),
        Event::Let,Event::Sleep(1.5),
        Event::Let,Event::Sleep(2.0),
        Event::Let,Event::Sleep(2.5),
    ]);
}