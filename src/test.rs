use super::*;

#[test]
fn test_parse_true() {
let src: &str = "true";
let p = NbParser::parse(Rule::term, &src)
.expect("err")
.next()
.unwrap();

let term = parse_term(p);
assert_eq!(term, Term::TmTrue);
}

#[test]
fn test_parse_false() {
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
