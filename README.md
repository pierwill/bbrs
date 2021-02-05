bbrs
====

Attempt at implementing a simple language in Rust using [`pest`](https://pest.rs).

Based on 𝔹 in Benjamin Pierce, *Types and Programming Languages* (U of Pennsylvania Press, 2002), page 34.

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
value = { "true" | "false" }
conditional = { "if" ~ term ~ "then" ~ term ~  "else" ~ term  }
var = @{ "$" ~ ASCII_DIGIT+ }
assignment = { var ~ "=" ~ value }
term = { value | conditional | assignment | var }
```

N.B.:
These are the [parsing expression grammars](https://en.wikipedia.org/wiki/Parsing_expression_grammar), used by `pest`,
not [Extended Backus–Naur form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

References
----------

- https://github.com/lazear/types-and-programming-languages
