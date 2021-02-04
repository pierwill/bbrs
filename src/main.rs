extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn main() {
    let input = "false";
    let p = NbParser::parse(Rule::term, &input)
        .expect("err")
        .next()
        .unwrap();
    let inner_rules = p.into_inner();
    println!("\nInner rules:\n{:#?}", inner_rules)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term() {
        let input = "true";
        let p = NbParser::parse(Rule::term, &input)
            .expect("err")
            .next()
            .unwrap();
        
        assert_eq!(p.into_inner().as_str(), "true");
    }

    #[test]
    fn test_if() {
        let input = "if true then true else false"
    }
}
