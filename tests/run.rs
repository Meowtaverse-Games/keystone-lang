use keystone_lang::*;

#[test]
fn main() {
    run(r#"
        print 100
        print 500
        print true
        print "Hello"
        print false
    "#);
}