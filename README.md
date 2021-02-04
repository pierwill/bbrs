Eventually I'd like things to look like this:


```
WHITESPACE = _{ " " }

value = { "true" | "false" }

conditional = { "if" ~ term ~ "then" ~ term ~  "else" ~ term  }

tvar = @{ "t" ~ ASCII_DIGIT+ }

assignment = { tvar ~ "=" ~ value }

term = { value | conditional | assignment | tvar }

statement = { term | assignment | tvar }
```