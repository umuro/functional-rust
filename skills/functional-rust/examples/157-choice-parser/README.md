# 157: Choice Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`alt` and `choice` try parsers in order and return the first one that succeeds — branching without `if`/`else`.

## The Problem This Solves

Most grammars have alternatives. A JSON value is *either* a string, *or* a number, *or* a boolean, *or* null, *or* an array, *or* an object. A language expression is *either* an identifier, *or* a literal, *or* a parenthesized sub-expression. You need a way to say "try A, and if A fails, try B."

Without a choice combinator, you'd write this by hand every time: run parser A, check if it failed, if yes run parser B, check if that failed too, etc. This is error-prone (easy to forget to reset the input position) and repetitive. `alt` and `choice` extract that pattern.

The critical property is *backtracking*: when parser A fails, we go back to the original input position before trying parser B. In Rust, this is free — `&str` is `Copy`, so holding the original `input` is just holding a value, no cloning required.

## The Intuition

`alt(p1, p2)` is like an `||` operator for parsers: "p1 OR p2." Try p1 first; if it fails, fall back to p2. If both fail, the whole thing fails.

`choice(vec![p1, p2, p3, ...])` extends this to any number of alternatives. It tries each parser left-to-right and returns the first success.

Important: order matters. `choice` is *ordered* — it tries parsers in the order you provide. If `tag("true")` comes before `tag("trueish")`, then `"trueish"` will match `"true"` and leave `"ish"` unconsumed. For this reason, put more specific alternatives before more general ones.

## How It Works in Rust

**`alt` — two alternatives:**
```rust
fn alt<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match p1(input) {
        Ok(result) => Ok(result),  // p1 succeeded — done
        Err(_)     => p2(input),   // p1 failed — try p2 from original position
    })
}
```
Notice `p2(input)` — not `p2(rest)`. We try p2 from the same starting position as p1. Since `input: &'a str` is `Copy`, this costs nothing.

**`choice` — list of alternatives:**
```rust
fn choice<'a, T: 'a>(parsers: Vec<Parser<'a, T>>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| {
        for parser in &parsers {
            if let Ok(result) = parser(input) {
                return Ok(result);  // first success wins
            }
            // failure: loop continues, trying from same input
        }
        Err("No parser matched".to_string())
    })
}
```
All parsers receive `input` (the original position) on each iteration. The `for` loop with `return` is the imperative equivalent of recursive backtracking.

**`alt_err` — accumulate error messages:**
```rust
fn alt_err<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match p1(input) {
        Ok(result) => Ok(result),
        Err(e1) => match p2(input) {
            Ok(result) => Ok(result),
            Err(e2)    => Err(format!("{} or {}", e1, e2)),  // combine both errors
        },
    })
}
```
When both fail, you get a message like `"Expected digit or Expected letter"` instead of just the last error. Better diagnostic messages = easier debugging.

**Usage:**
```rust
// Boolean literal parser
let bool_p = alt(tag("true"), tag("false"));
println!("{:?}", bool_p("true!"));  // Ok(("true", "!"))
println!("{:?}", bool_p("false!")); // Ok(("false", "!"))
println!("{:?}", bool_p("maybe"));  // Err("Expected \"false\"")

// JSON primitive parser
let json_atom = choice(vec![tag("true"), tag("false"), tag("null")]);
println!("{:?}", json_atom("null!")); // Ok(("null", "!"))
```

## What This Unlocks

- **Any grammar with variants** — keywords, operators, literals — all expressed as `alt` or `choice`.
- **Ordered PEG grammars** — unlike context-free grammars where ambiguity needs resolution, parser combinators use the order you specify to resolve it deterministically.
- **Error reporting** — `alt_err` accumulates all attempted alternatives in the error, making it clear what was expected at each position.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Two-way choice | `alt p1 p2` or `p1 <\|> p2` | `alt(p1, p2)` |
| List of choices | Recursive over a list | `Vec<Parser<'a, T>>` with `for` loop |
| Backtracking cost | Free (immutable strings, GC) | Free (`&str` is `Copy` — just a pointer) |
| Error accumulation | String concatenation in `Err` | `format!("{} or {}", e1, e2)` |
| Order sensitivity | Yes — first match wins | Yes — first match wins |
