use keystone_lang::*;

#[test]
fn main() {
    run(r#"
        print 1*2+3
        print 500
        print true
        print 5 + 5
        print "Hello"
        print 1 < 1 or 1==1
        print false
        move "Hello"
        turn left
    "#);
}