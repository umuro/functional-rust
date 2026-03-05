📖 **[View on hightechmind.io →](https://hightechmind.io/rust/158-sequence-parser)**

---

# 158: Sequence Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`pair`, `preceded`, `terminated`, and `delimited` run parsers one after another, keeping only the results you care about.

## The Problem This Solves

Real grammar elements are sequences: a key-value pair is `key ":" value`. A parenthesized expression is `"(" expr ")"`. A terminated statement is `statement ";"`. You need parsers that run in sequence, thread the remaining input through each step, and let you choose which results to keep.

Without sequence combinators, you'd write nested pattern matches: run p1, get `(v1, r1)`, run p2 on `r1`, get `(v2, r2)`, run p3 on `r2`, etc. Each step manually threads the remainder. That's mechanical repetition — exactly what a combinator should eliminate.

`pair` keeps both results. `preceded` runs two parsers but discards the first (the "prefix"). `terminated` keeps the first and discards the second (the "suffix"). `delimited` keeps only the middle, discarding both open and close delimiters. These are the four combinations of "keep or discard" for two-sided sequencing.

## The Intuition

Think of `delimited(tag("("), expr, tag(")"))` as: "require a `(`, parse `expr`, require a `)`, give me `expr`." You care about the content, not the brackets. The brackets are purely syntactic scaffolding.

`preceded` is "require X before Y, give me Y." `terminated` is "give me X, then require Y." Both are `delimited` in disguise, just with one side dropped.

The Rust `?` operator makes sequencing elegant: each step either succeeds (binding the value and remainder) or immediately propagates the error up. No nested matches needed.

## How It Works in Rust

**`pair` — run two parsers, keep both:**
```rust
fn pair<'a, A: 'a, B: 'a>(p1: Parser<'a, A>, p2: Parser<'a, B>) -> Parser<'a, (A, B)> {
    Box::new(move |input: &'a str| {
        let (v1, rest) = p1(input)?;   // run p1; ? propagates Err
        let (v2, remaining) = p2(rest)?; // run p2 on what's left
        Ok(((v1, v2), remaining))
    })
}
```
`?` is the key — it's equivalent to `match { Ok(x) => x, Err(e) => return Err(e) }`. Each step only runs if the previous one succeeded, and threading `rest` to `p2` ensures parsers advance sequentially.

**`preceded` — discard prefix, keep suffix:**
```rust
fn preceded<'a, A: 'a, B: 'a>(prefix: Parser<'a, A>, p: Parser<'a, B>) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = prefix(input)?;  // run prefix, discard its value (underscore)
        p(rest)                          // run p on remainder, return its result
    })
}
```

**`terminated` — keep value, discard suffix:**
```rust
fn terminated<'a, A: 'a, B: 'a>(p: Parser<'a, A>, suffix: Parser<'a, B>) -> Parser<'a, A> {
    Box::new(move |input: &'a str| {
        let (value, rest) = p(input)?;         // parse the thing we care about
        let (_, remaining) = suffix(rest)?;    // parse (and discard) the suffix
        Ok((value, remaining))                 // return the value, not the suffix
    })
}
```

**`delimited` — keep middle, discard both sides:**
```rust
fn delimited<'a, A: 'a, B: 'a, C: 'a>(
    open: Parser<'a, A>,
    p: Parser<'a, B>,
    close: Parser<'a, C>,
) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, r1) = open(input)?;   // consume open delimiter
        let (value, r2) = p(r1)?;     // parse content
        let (_, r3) = close(r2)?;     // consume close delimiter
        Ok((value, r3))               // return only the content
    })
}
```

**Usage:**
```rust
// Parse (x) and return x
let p = delimited(tag("("), satisfy(|c| c.is_ascii_lowercase(), "letter"), tag(")"));
println!("{:?}", p("(x)rest")); // Ok(('x', "rest"))

// Parse "key: value" and return both
let p = pair(tag("key"), preceded(tag(": "), tag("value")));
println!("{:?}", p("key: value")); // Ok((("key", "value"), ""))

// Parse "stmt;" and return "stmt"
let p = terminated(tag("return"), tag(";"));
println!("{:?}", p("return;")); // Ok(("return", ""))
```

## What This Unlocks

- **Bracketed expressions** — `delimited(tag("("), expr, tag(")"))` handles parentheses, brackets, braces universally.
- **Token separation** — `terminated(key, tag(":"))` strips the colon; `preceded(tag("let "), ident)` strips the keyword.
- **Structured data** — chain `pair` calls (or use `triple`) to parse tuples, records, and structured tokens.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error propagation | Nested `match` / `bind` | `?` operator — cleaner, same semantics |
| Tuple return | `('a * 'b)` | `(A, B)` |
| Discarding values | `let _ = ...` | `let (_, rest) = ...` |
| Composability | Manual nesting in practice | `?` chains read top-to-bottom |
| Three-way sequence | Explicit nested pair | `triple(p1, p2, p3)` returning `(A, B, C)` |
