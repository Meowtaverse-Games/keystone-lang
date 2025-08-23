// tests/integration_test.rs
use keystone_lang::*;

#[test]
fn main() {
    run("
        hogehoge = 1
        print hogehoge + 2
        if hogehoge < 2 then
            print hogehoge
        end
        loop 3 times
            print hogehoge
        end
    ");
}