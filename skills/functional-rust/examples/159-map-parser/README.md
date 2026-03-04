# 159: Map Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`map(parser, f)` transforms the parsed value with a function — turning raw characters and strings into typed data.

## The Problem This Solves

Parsing produces raw text. What you actually want is typed data. You parse `"42"` but you want the number `42`. You parse `"true"` but you want the boolean `true`. You parse a list of digit characters `['1', '2', '3']` but you want the integer `123`.

Every parser eventually needs to convert its raw output into something useful. Without `map`, you'd run the parser, get the result, then manually transform it outside the parser. That means you can't compose — the transformation isn't part of the parser chain. With `map`, the transformation lives *inside* the parser, so you can pass a `Parser<u64>` to anything expecting a parser, not a `Parser<Vec<char>>` that needs manual post-processing.

`map` is what makes parsers *functors* — a concept from functional programming that means "you can transform the inside value without changing the structure." Same parser, same position tracking, different output type.

## The Intuition

`map` is `Iterator::map` but for parsers. You already know `vec.iter().map(|x| x * 2)` — it applies a function to each element without touching the collection structure. `map(parser, f)` does the same: apply `f` to the parsed value, without changing what the parser consumes or how it handles errors.

If the parser fails, `map` fails too — it only runs `f` on success. The transformation is "inside" the `Result`. This makes `map` pure: it never changes parsing behavior, only the output type.

## How It Works in Rust

**`map` — transform the parsed value:**
```rust
fn map<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> B + 'a  // f must be callable with A, produce B, and be capturable
{
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;  // run parser; propagate error
        Ok((f(value), rest))                 // transform value, keep same remainder
    })
}
```
The generic parameters `A` and `B` let `map` work with any input/output type pair. `F: Fn(A) -> B + 'a` says "any function (or closure) from A to B."

**`map_const` — replace parsed value with a constant:**
```rust
fn map_const<'a, A: 'a, B: Clone + 'a>(parser: Parser<'a, A>, value: B) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = parser(input)?;   // run parser, discard its value
        Ok((value.clone(), rest))         // return fixed constant instead
    })
}
```
This is how you turn `tag("true")` into a parser that returns `true` (boolean), not `"true"` (string).

**`parse_nat` — a practical composition:**
```rust
fn parse_nat<'a>() -> Parser<'a, u64> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),  // parse: Vec<char>
        |digits| digits.iter().fold(0u64, |acc, &d| {
            acc * 10 + (d as u64 - '0' as u64)  // convert digit char to value
        }),
    )
}
```
Step by step:
1. `satisfy(|c| c.is_ascii_digit(), "digit")` — parses one digit char
2. `many1(...)` — collects one or more into `Vec<char>`
3. `map(..., |digits| fold(...))` — converts `Vec<char>` to `u64`
4. Result: a `Parser<u64>` — fully typed, fully composed

**Usage:**
```rust
// char → uppercase
let p = map(satisfy(|c| c.is_ascii_lowercase(), "lower"), |c| c.to_ascii_uppercase());
println!("{:?}", p("abc")); // Ok(('A', "bc"))

// "true" → true (boolean)
let p = map_const(tag("true"), true);
println!("{:?}", p("true!")); // Ok((true, "!"))

// digit string → u64
let p = parse_nat();
println!("{:?}", p("42rest")); // Ok((42, "rest"))

// Compose maps: char → u32 → u32 (doubled)
let p = map(
    map(satisfy(|c| c.is_ascii_digit(), "digit"), |c| c as u32 - '0' as u32),
    |n| n * 2,
);
println!("{:?}", p("5")); // Ok((10, ""))
```

## What This Unlocks

- **Typed parse results** — your parsers return `u64`, `bool`, `String`, `Vec<Token>` — not `char` or `&str`.
- **Composition without breaking the chain** — transformations live inside the parser, so you can pass `parse_nat()` wherever a `Parser<u64>` is expected.
- **The `parse_nat` pattern** is used in examples 160, 161, and 162 — it's the standard way to parse numbers in the entire series.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Functor syntax | `map f p` (function first, like `f <$> p`) | `map(p, f)` (parser first) |
| Generic bounds | Inferred by type checker | `F: Fn(A) -> B + 'a` explicit |
| Digit conversion | `Char.code c - Char.code '0'` | `c as u64 - '0' as u64` |
| Composed maps | Natural with currying: `map (map p f) g` | Explicit nesting: `map(map(p, f), g)` |
| Constant replacement | `map (fun _ -> v) p` | `map_const(p, v)` |
