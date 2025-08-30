use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct KeystoneLang;

pub fn analyze() {
    let input = r#"
        x = 1 + 2
        print x
        loop 2
            move "forward"
            print "hello"
        end
    "#;

    match KeystoneLang::parse(Rule::program, input) {
        Ok(pairs) => {
            for pair in pairs {
                println!("Rule: {:?}", pair.as_rule());
                println!("Text: {:?}", pair.as_str());

                for inner in pair.into_inner() {
                    println!("  Child Rule: {:?}", inner.as_rule());
                    println!("  Child Text: {:?}", inner.as_str());
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}
