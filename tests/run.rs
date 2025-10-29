use keystone_lang::eval;

#[test]
fn main() {
  println!("{:?}",
    eval(r#"
        hogehoge = 1
        hogehoge2 = "hello"

        if hogehoge < 1
          print "1: hogehoge < 1"
        end

        if hogehoge > 1
          print "1: hogehoge > 1"
        end

        if hogehoge == 1
          print "1: hogehoge == 1"
        end

        hogehoge = 2

        if hogehoge < 1
          print "2: hogehoge < 1"
        end

        if hogehoge > 1
          print "2: hogehoge > 1"
        end

        if hogehoge == 1
          print "2: hogehoge == 1"
        end

        a = 989
        print a

        loop 2
            print a
            print "あいうえお"
            move left
        end

        d = 3

    "#)
  );
}