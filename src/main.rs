#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused)]
use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn main() {

    let input = "true";
    
    let p = NbParser::parse(Rule::statement, &input)
        .expect("err")
        .next()
        .unwrap();
    println!("{:#?}", p);
        
    let inner_rules = p.into_inner();
    println!("{:#?}", inner_rules);
    
    match inner_rules.clone().next().unwrap().as_rule() {
        Rule::term => println!("it's a term"),
        Rule::assignment => println!("It's an assignment"),
        Rule::tvar => println!("It's a tvar"),
        _ => panic!()
    }
}
