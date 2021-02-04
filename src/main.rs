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

    let _term = parse_term(p);
    println!("{:#?}", _term);

}

fn parse_term(p: Pair<'_, Rule>) -> Term {
    let inner_pair = p.into_inner();
    let first_str_of_inner = inner_pair.as_str().split_whitespace().collect::<Vec<_>>()[0];
    match first_str_of_inner {
        "true" => Term::TmTrue,
        "false" => Term::TmFalse,
        "if" => Term::TmIf(_,_,_),
    }
}

// fn parse_if(t: Term) -> Option(Term) {}

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

fn main() {
    println!("λ");
    let input = "if iszero(succ(zero)) then false else succ(4)";
    let mut p = Parser::new(input);
    while let Some(tm) = p.parse_term() {
        print!("{:?} ==> ", tm);
        println!("{:?}", eval(tm));
    }
}

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let src: &str = "false";
        // let src: &str = "if true then true else false";
        let p = NbParser::parse(Rule::term, &src)
            .expect("err")
            .next()
            .unwrap();

        let term = parse_term(p);
        assert_eq!(term, Term::TmFalse);
    }
}
