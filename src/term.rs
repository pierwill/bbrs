extern crate pest;

use std::fmt;

use pest::iterators::{Pair, Pairs};
use crate::Rule;

/// Takes a [`Pair`] and returns a [`Term`].
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

/// Parse an “if” term (aka an “if statement”).
///
/// See <https://github.com/lazear/types-and-programming-languages/blob/master/01_arith/src/parser.rs#L54-L61>
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

/// A term in the grammar.
#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    TmTrue,
    TmFalse,
    TmIf(Box<Term>, Box<Term>, Box<Term>),
}

impl Term {
    // Hack to get boolean normal terms from their string representations.
    pub fn from_str(s: &str) -> Self {
        match s {
            "true" => Term::TmTrue,
            "false" => Term::TmFalse,
            _ => panic!(),
        }
    }

    /// Returns true if the term is in normal (irreducible) form.
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

