extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

const SEPARATOR: &str = "\n----------------------------------\n";

fn main() {
    let input = "false";
    let p = NbParser::parse(Rule::statement, &input)
        .expect("err")
        .next()
        .unwrap();
    let inner_rules = p.into_inner();
    println!("Input: \"{}\"\n\n{:#?}", input, inner_rules);

    println!("{}", SEPARATOR);

    let input2 = "t1 = true";
    let p2 = NbParser::parse(Rule::statement, &input2)
        .expect("err")
        .next()
        .unwrap();
    println!("Input: \"{}\"\n\n{:#?}", input2, p2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term() {
        let input = "true";
        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();

        assert_eq!(p.into_inner().as_str(), "true");
    }

    #[test]
    fn test_term2() {
        let input = "true";
        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();
        
        assert_eq!(p.into_inner().next().unwrap().as_rule(), Rule::term);
    }
    
    #[test]
    fn test_assignment_str() {
        let input = "t1 = true";
        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();
        assert_eq!(p.into_inner().as_str(), "t1 = true");
    }

    #[test]
    fn test_assignment_rule() {
        let input = "t1 = true";
        let p = NbParser::parse(Rule::statement, &input)
            .expect("err")
            .next()
            .unwrap();
        assert_eq!(p.into_inner().next().unwrap().as_rule(), Rule::assignment);
    }
}
