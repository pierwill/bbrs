use crate::term::*;
use crate::Rule;
use crate::BbParser;
use pest::Parser;
use pest::iterators::Pairs;

#[test]
fn test_parse_true() {
    let src: &str = "true";
    let p = BbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();

    let term = parse_term(p);
    assert_eq!(term, Term::True);
}

#[test]
fn test_parse_false() {
    let src: &str = "false";
    let p = BbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();

    let term = parse_term(p);
    assert_eq!(term, Term::False);
}

#[test]
fn test_parse_if() {
    let src: &str = "if true then true else false";
    let p = BbParser::parse(Rule::term, &src)
        .expect("err")
        .next()
        .unwrap();

    let term = parse_if(Pairs::single(p));
    let want = Some(Term::If(Box::new(Term::True), Box::new(Term::True), Box::new(Term::False)));
    assert_eq!(term, want);
}
