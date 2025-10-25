use keystone_lang::*;

#[test]
fn main() {
    run(r#"
        print 1*2+3
        print 500
        ct = 300
        loop ct
            print "Hey"
            print "who"
        end
        if ct < 100
            print "yap"
        end
        print true
        print 5 + 5
        print "Hello"
        print 1 < 1 or 1==1
        print false
        name = right
        turn hoge
    "#);
}