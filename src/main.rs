#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn main() {}

struct Assignment {
    // eg "t1"
    tvar: &'static str,
    val: Value,
}

impl Assignment {}

struct Term {
    kind: TmKind,
}

enum TmKind {
    Value,
    Conditional,
}

enum Value {
    TRUE,
    FALSE,
}

enum AstNode {
    Assignment,
    Term,
    TmKind,
    Value(Value),
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, pest::error::Error<Rule>> {
    let mut ast = vec![];

    let pairs = NbParser::parse(Rule::statement, &source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                ast.push(build_ast_from_statement(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}

fn build_ast_from_statement(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::statement => build_ast_from_statement(pair.into_inner().next().unwrap()),
        Rule::value => {
            let mut pair = pair.into_inner();
            let val = pair.next().unwrap().as_str();
            match val {
                "true" => AstNode::Value(Value::TRUE),
                "false" => AstNode::Value(Value::FALSE)
            }
        }
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let input = "true";
        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();

        assert_eq!(build_ast_from_statement(p), "true");
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_term() {
//         let input = "true";
//         let p = NbParser::parse(Rule::statement, &input)
//             .expect("err")
//             .next()
//             .unwrap();

//         assert_eq!(p.into_inner().as_str(), "true");
//     }

//     #[test]
//     fn test_term2() {
//         let input = "true";
//         let p = NbParser::parse(Rule::statement, &input)
//             .expect("err")
//             .next()
//             .unwrap();

//         assert_eq!(p.into_inner().next().unwrap().as_rule(), Rule::term);
//     }

//     #[test]
//     fn test_assignment_str() {
//         let input = "t1 = true";
//         let p = NbParser::parse(Rule::statement, &input)
//             .expect("err")
//             .next()
//             .unwrap();
//         assert_eq!(p.into_inner().as_str(), "t1 = true");
//     }

//     #[test]
//     fn test_assignment_rule() {
//         let input = "t1 = true";
//         let p = NbParser::parse(Rule::statement, &input)
//             .expect("err")
//             .next()
//             .unwrap();
//         assert_eq!(p.into_inner().next().unwrap().as_rule(), Rule::assignment);
//     }

//     #[test]
//     fn test_val() {
//         let input = "true";
//         let p = NbParser::parse(Rule::statement, &input)
//             .expect("err")
//             .next()
//             .unwrap();
//         assert_eq!(
//             String::from(p.into_inner().next().unwrap().as_str()),
//             eval_t(input)
//         );
//     }
// }
