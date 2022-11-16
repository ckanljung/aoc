use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "bag.pest"]
pub struct BagParser;

fn get_bagcolor(a : Pair) -> (String,Option<String>) {
    return ("".to_string(),None);
}

fn countbags(bagfile : &str) -> Option<i32> {
    let unparsed_file = fs::read_to_string(bagfile).expect("cannot read file");

    let mut current_section_name = "";
    
    let file = BagParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        println!("Line: {:?}", line);
        match line.as_rule() {
            Rule::line => {
                let (rest,bagcolor) = get_bagcolor(line);
                match bagcolor {
                    Some(a) => {
                    }
                    None => {
                        return None;
                    },
                }
            }
            /*
                for elem in line.into_inner() {
                    let mut bag = get_bag(elem);
                    match elem.as_rule() {
                        Rule::bag => {
                            println!("### aaa: {:?}", a);
                        }
                        Rule::bag => {
                    }
                }
                let mut inner_rules = line.into_inner(); // { name }
                current_section_name = inner_rules.next().unwrap().as_str();
                println!("### Section: {}", current_section_name);
                */
            /*
            Rule::property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();

                // Insert an empty inner hash map if the outer hash map hasn't
                // seen this section name before.
                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            */
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    return None;
}

fn main() {
    assert_eq!(countbags("test2.txt"), Some(4));
    println!("Hello, world!");
}
