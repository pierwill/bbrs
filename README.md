[![Rust](https://github.com/pierwill/bbrs/actions/workflows/rust.yml/badge.svg)](https://github.com/pierwill/bbrs/actions/workflows/rust.yml)

bbrs
====

`bbrs` implements a simple language from Pierce's TAPL in Rust using [`pest`](https://pest.rs).

Based on **𝔹** in Benjamin Pierce, *Types and Programming Languages* (U of Pennsylvania Press, 2002), page 34.

Currently working on implementing this grammar:

```
value = { "true" | "false" }
conditional = { "if" ~ term ~ "then" ~ term ~  "else" ~ term  }
term = { value | conditional }
```

The idea is a working, extensible interprer that'll do something like this:

```
>> true
true
>> false
false
>> if true then true else false
true
>> if false then true else false
false
>> funny
Error!
```

Eventually I'd like to add named variables and assignments.
Things would look something like this:

```
WHITESPACE = _{ " " }
value = { "true" | "false" }
conditional = { "if" ~ term ~ "then" ~ term ~ "else" ~ term  }
name = @{ !("let"|"if"|"then"|"else"|value) ~ "_"? ~ ASCII_ALPHA+ ~ ANY? }
assignment = { "let" ~ name ~ "=" ~ value }
term = { name | assignment | value | conditional }
```

N.B.:
These are [parsing expression grammars](https://en.wikipedia.org/wiki/Parsing_expression_grammar) as used by `pest`,
not [Extended Backus–Naur form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

References
----------

- https://github.com/lazear/types-and-programming-languages
