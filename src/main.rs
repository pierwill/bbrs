#![allow(dead_code)]

use std::fmt;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused)]
use pest::{Parser, iterators::{Pair, Pairs}};

#[derive(Parser)]
#[grammar = "nb.pest"]
struct NbParser;

fn main() {
    // let src: &str = "true";
    let src: &str = "if false then false else true";

    let p = NbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();
    // println!("{:#?}", p);
    
    // let _term = parse_term(p.clone());
    // println!("{:#?}", _term);

    let t = parse_term(p);
    println!("{}", eval(t));
}

pub fn parse_term(p: Pair<'_, Rule>) -> Term {
    let inner_pair = p.into_inner();
    let first_str = inner_pair.as_str().split_whitespace().collect::<Vec<_>>()[0];

    match first_str {
        "if" => parse_if(inner_pair).unwrap(),
        "true" => Term::TmTrue,
        "false" => Term::TmFalse,
        _ => panic!(),
    }
}

pub fn parse_if(p: Pairs<'_, Rule>) -> Option<Term> {
    // "if cond then csq else alt"
    let conditional_str_parts = p.as_str().split_whitespace().collect::<Vec<_>>();

    let cond = Term::from_str(
        conditional_str_parts[1]
    );
    debug_assert_eq!(conditional_str_parts[2], "then");
    let csq = Term::from_str(
        conditional_str_parts[3]
    );
    debug_assert_eq!(conditional_str_parts[4], "else");
    let alt = Term::from_str(
        conditional_str_parts[5]
    );
    
    Some(Term::TmIf(Box::new(cond), Box::new(csq), Box::new(alt)))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    TmTrue,
    TmFalse,
    TmIf(Box<Term>, Box<Term>, Box<Term>),
}

impl Term {
    pub fn from_str(s: &str) -> Self {
        match s {
            "true" => Term::TmTrue,
            "false" => Term::TmFalse,
            _ => panic!(),
        }
    }

    pub fn is_normal(&self) -> bool {
        match self {
            Term::TmTrue | Term::TmFalse => true,
            _ => false,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Term::TmTrue => "true",
            Term::TmFalse => "false",
            _ => panic!("We shouldn't need to print an if.")
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    NoRuleApplies,
}

pub fn eval1(t: Term) -> Result<Term, RuntimeError> {
    let res = match t {
        Term::TmIf(cond, csq, alt) => match *cond {
            Term::TmFalse => *alt,
            Term::TmTrue => *csq,
            _ => Term::TmIf(Box::new(eval1(*cond)?), csq, alt),
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

        let term = parse_if(Pairs::single(p));
        let want = Some(Term::TmIf(Box::new(Term::TmTrue), Box::new(Term::TmTrue), Box::new(Term::TmFalse)));
        assert_eq!(term, want);
    }
}
