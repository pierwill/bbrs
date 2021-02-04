#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused)]
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn consume(src: &str) -> String {
    let mut result = Vec::new();

    let pairs = NbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();

    for pair in pairs.into_inner() {
        // Match the rule.
        match pair.as_rule() {
            // If it is a value, return the value.
            Rule::value => result.push(pair.as_str()),

            // If it is a conditional, evaluate it using E-If.
            Rule::conditional => result.push(e_if(pair.as_str())),

            // None applies.
            _ => unimplemented!(),
        };
    }

    result.into_iter().collect()
}

/// Refers to E-If evaluation rule in Figure 3-1 in Pierce et al. 2002
fn e_if(src: &str) -> &str {
    let pairs = NbParser::parse(Rule::conditional, &src)
        .expect("err")
        .next()
        .unwrap();
    let conditional_terms = pairs.into_inner().collect::<Vec<Pair<Rule>>>();

    let t_1: &Pair<Rule> = &conditional_terms[0];
    let t_2: &Pair<Rule> = &conditional_terms[1];
    let t_3: &Pair<Rule> = &conditional_terms[2];

    if t_1.as_str() == "true" {
        return t_2.as_str();
    } else {
        return t_3.as_str();
    }
}

fn main() {
    // leaving this as is for interactive testing
    //let src: &str = "false";
    let src: &str = "if true then true else false";
    let result = consume(&src);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;

    // TODO: Add test_true and test_false to assert that
    // parsing "false" or "true" results in just those values
    // without errors
    #[test]
    fn test_if_true() {
        let src: &str = "if true then true else false";
        assert_eq!(consume(src), "true".to_string());
    }

    #[test]
    fn test_if_false() {
        let src: &str = "if false then true else false";
        assert_eq!(consume(src), "false".to_string());
    }
}
