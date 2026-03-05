📖 **[View on hightechmind.io →](https://hightechmind.io/rust/155-many-parser)**

---

# 155: Many Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`many0` and `many1` repeat a parser until it fails — the parser equivalent of regex `*` and `+`.

## The Problem This Solves

Single-shot parsers are useful, but most real tokens are sequences: a number is many digits, an identifier is many letters-and-underscores, a string is many non-quote characters. Every real parser needs "repeat this until it stops working."

Without a repetition combinator, you'd need to manually write a loop every time: call the parser, check if it succeeded, if yes accumulate the result and call again, if no stop. That loop is identical for every repeating pattern — it belongs in a combinator, not duplicated across your codebase.

`many0` (zero or more) and `many1` (one or more) are those combinators. They're the difference between writing a number parser in one line versus ten.

## The Intuition

`many0` is like a greedy regex `*`: keep going as long as the parser succeeds, stop the moment it fails. Crucially, zero successes is still a success — `many0` never returns an error. If the parser fails on the first try, you just get an empty `Vec`.

`many1` is like regex `+`: same as `many0`, but requires at least one match. If the very first attempt fails, `many1` fails too.

`many_till` adds a terminator: "keep parsing until *this other parser* succeeds." Useful for parsing strings ("take chars until you hit a `"`") or comments ("take chars until you hit `*/`").

## How It Works in Rust

**`many0` — zero or more:**
```rust
fn many0<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        while let Ok((value, rest)) = parser(input) {
            results.push(value);
            input = rest;   // advance: next iteration starts where this one left off
        }
        Ok((results, input))  // always Ok — even if results is empty
    })
}
```
`while let Ok(...) = parser(input)` is the idiom: destructure the success, or break on error. The `mut input` variable acts as a cursor — we rebind it to `rest` each iteration. No recursion, no `List.rev`, just a simple loop.

**`many1` — one or more:**
```rust
fn many1<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = parser(input)?;  // ? propagates failure immediately
        let mut results = vec![first];
        while let Ok((value, rest)) = parser(remaining) {
            results.push(value);
            remaining = rest;
        }
        Ok((results, remaining))
    })
}
```
The `?` on the first parse does the work: if the parser fails even once, we return `Err` immediately. After that, it's `many0` logic.

**`many_till` — parse until a terminator:**
```rust
fn many_till<'a, T: 'a, U: 'a>(
    parser: Parser<'a, T>,
    stop: Parser<'a, U>,
) -> Parser<'a, (Vec<T>, U)> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        loop {
            if let Ok((term, rest)) = stop(input) {
                // Terminator matched — return everything collected plus the terminator
                return Ok(((results, term), rest));
            }
            // Terminator didn't match — try the main parser; fail if it can't advance either
            let (value, rest) = parser(input)?;
            results.push(value);
            input = rest;
        }
    })
}
```

**`many0_str` — collect chars into a `String`:**
```rust
fn many0_str<'a>(parser: Parser<'a, char>) -> Parser<'a, String> {
    Box::new(move |mut input: &'a str| {
        let mut s = String::new();
        while let Ok((c, rest)) = parser(input) {
            s.push(c);
            input = rest;
        }
        Ok((s, input))
    })
}
```

**Usage:**
```rust
let digits = many0(satisfy(|c| c.is_ascii_digit(), "digit"));
println!("{:?}", digits("123abc")); // Ok((vec!['1','2','3'], "abc"))
println!("{:?}", digits("abc"));    // Ok((vec![], "abc")) — zero, still Ok

let digits1 = many1(satisfy(|c| c.is_ascii_digit(), "digit"));
println!("{:?}", digits1("abc"));   // Err — at least one required

let digit_str = many0_str(satisfy(|c| c.is_ascii_digit(), "digit"));
println!("{:?}", digit_str("456xy")); // Ok(("456", "xy"))
```

## What This Unlocks

- **Practical token parsers** — numbers, identifiers, strings, whitespace — all require "repeat until fail."
- **`many1(satisfy(is_digit))` + `map`** is how `parse_nat` (example 159) works: collect digit chars, then fold into a `u64`.
- **`many_till`** enables parsing balanced delimiters, comments, and string literals without scanning for the terminator manually.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Loop style | Tail-recursive `go` helper | `while let Ok(...) = ...` loop |
| Result accumulation | Reverse a list at the end (`List.rev`) | `Vec::push` in order — no reverse needed |
| Always succeeds | `many0` returns `Ok ([], input)` | `many0` returns `Ok((vec![], input))` |
| String collection | `String.concat` from char list | `String::push` directly in loop |
| Early exit (`many1`) | Pattern match on first call | `?` operator on first call |
