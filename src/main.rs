#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused)]
use pest::{Parser, iterators::{Pair, Pairs}};

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn main() {

    let src: &str = "false";
    // let src: &str = "if true then true else false";
    
    let p = NbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();

    // Get the first inner pair.
    // What kind of term is it?
    let inner: Pairs<'_, Rule> = p.into_inner();

    // What kind of rule is the first inner pair (term)?
    let inner_rule = inner.peek().unwrap().as_rule();

    // Match the rule.
    let m = match inner_rule {
        
        // If it is a value, return the value.
        Rule::value => unimplemented!(),
        
        // If it is a conditional, evaluate it.
        Rule::conditional => unimplemented!(),

        // None applies.
        _ => unimplemented!()
    };

    println!("{:#?}", m)
}
