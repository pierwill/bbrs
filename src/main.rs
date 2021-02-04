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

    let inputs: Vec<&str> = vec![
        "true",
        "false",
        // "t3 = true",
        // "t1989",
        "if t1 then t2 else t3",
    ];

    for input in inputs {

        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();

        let p2 = p.clone().into_inner().next().unwrap();
        let p3 = p2.clone().into_inner().next().unwrap();
        let p4 = p3.clone().into_inner().next(); // don't unwrap None

        println!("{:?}", input);
        println!("{:?}", p.as_rule());
        println!("{:?}", p2.as_rule());
        println!("{:?}:{}", p3.as_rule(), p3.as_str());
        println!("{:?}\n", p4);
    }
}


// fn recurse(p: Pair<_>) -> Option<Pair<_>> {
//     let inner = p.clone().into_inner().next();
//     let inner_rule = inner.as_rule();
//     match inner_rule {
//         Rule::statement => Some(inner),
//         Rule::term => Some,
//         Rule::value =>,
//     }
// }

// struct Term {
//     info: &'static str,
//     kind: TermKind,
// }

// enum TermKind {
//     Value(ValueKind),
//     Conditional,
//     Assignment,
// }

// enum ValueKind {
//     True,
//     False,
// }

// enum AstNode {
//     Value,
//     Conditional,
//     Tvar,
//     Assignment,
//     Term
// }
