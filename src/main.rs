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
    println!("{:#?}", p);
    
    let _term = parse_term(p);
    println!("{:#?}", _term);

}

fn parse_term(p: Pair<'_, Rule>) -> Term {
    let inner_pair = p.into_inner();
    let first_str = inner_pair.as_str().split_whitespace().collect::<Vec<_>>()[0];

    match first_str {
        "if" => parse_if(inner_pair).unwrap(),
        "true" => Term::TmTrue,
        "false" => Term::TmFalse,
        _ => panic!(),
    }
}

fn parse_if(p: Pairs<'_, Rule>) -> Option<Term> {

    // "if cond then csq else alt"
    let conditional_str_parts = Pairs::single(p).as_str().split_whitespace().collect::<Vec<_>>();

    let cond = Term::from_str(
        conditional_str_parts[1]
    );
    debug_assert_eq!(conditional_str_parts[3], "then");
    let csq = Term::from_str(
        conditional_str_parts[3]
    );
    debug_assert_eq!(conditional_str_parts[5], "else");
    let alt = Term::from_str(
        conditional_str_parts[5]
    );
    
    Some(Term::TmIf(Box::new(cond), Box::new(csq), Box::new(alt)))
}

// fn evaluate(t: Term) -> Result<Term, RuntimeError> {
//     let r = match t {
//         Tm
//     }
// }

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Term {
    TmTrue,
    TmFalse,
    TmIf(Box<Term>, Box<Term>, Box<Term>),
}

impl Term {
    fn from_str(s: &str) -> Self {
        match s {
            "true" => Term::TmTrue,
            "false" => Term::TmFalse,
            _ => panic!(),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RuntimeError {
    NoRuleApplies,
}

// fn parse_if(&mut self) -> Option<Term> {
//     let cond = self.parse_term()?;
//     let _ = self.expect(Token::Then)?;
//     let csq = self.parse_term()?;
//     let _ = self.expect(Token::Else)?;
//     let alt = self.parse_term()?;
//     Some(Term::TmIf(Box::new(cond), Box::new(csq), Box::new(alt)))
// }

/*
pub fn eval1(t: Term) -> Result<Term, RuntimeError> {
    let res = match t {
        TmIf(cond, csq, alt) => match *cond {
            TmFalse => *alt,
            TmTrue => *csq,
            _ => TmIf(Box::new(eval1(*cond)?), csq, alt),
        },
        _ => return Err(RuntimeError::NoRuleApplies),
    };
    Ok(res)
}

pub fn eval(t: Term) -> Term {
    let mut r = t;
    while let Ok(tprime) = eval1(r.clone()) {
        r = tprime;
        if r.is_normal() {
            break;
        }
    }
    r
}

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let src: &str = "false";
        let p = NbParser::parse(Rule::term, &src)
            .expect("err")
            .next()
            .unwrap();

        let term = parse_term(p);
        assert_eq!(term, Term::TmFalse);
    }

    #[test]
    fn test_parse_if() {
        let src: &str = "if true then true else false";
        let p = NbParser::parse(Rule::term, &src)
            .expect("err")
            .next()
            .unwrap();

        let term = parse_if(p);
        assert_eq!(term, Some(Term::TmIf(Box::new(Term::TmTrue), Box::new(Term::TmTrue), Box::new(Term::TmFalse))));
    }



}
