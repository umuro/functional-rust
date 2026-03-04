# 156: Optional Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`opt(p)` wraps any parser to make it optional — the parser equivalent of regex `?`.

## The Problem This Solves

Some parts of a grammar are optional. A number might have a sign: `-42` or `+42` or just `42`. An HTTP header might have a comment in parentheses or might not. A function parameter list might be empty.

Without `opt`, you'd need to handle this with awkward fallback logic everywhere: try the parser, catch the error, decide whether to continue or not. That logic is always the same — "try it, use the value if it works, use `None` if it doesn't, never fail" — so it belongs in a combinator.

`opt` converts a parser that *might fail* into a parser that *never fails*, returning `Some(value)` on success and `None` on failure. The key property: on failure, the input is left unchanged. The optional thing simply wasn't there.

## The Intuition

Think of `opt` as "try this, don't panic if it doesn't work." If you ask a friend "did you bring an umbrella?" and they say no, you don't stop the conversation — you just know the umbrella isn't there. `opt` is that shrug.

`peek` is a related idea: "look ahead without moving." After a successful `peek`, the input position stays where it was. You saw the next character, but didn't consume it. Useful for deciding which parser to use next without committing.

`with_default` is the opposite of `None` — if the optional thing wasn't there, assume a default value. Instead of `Option<T>`, you get `T` directly.

## How It Works in Rust

**`opt` — make any parser optional:**
```rust
fn opt<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, rest)) => Ok((Some(value), rest)),  // success: wrap in Some
        Err(_)            => Ok((None, input)),         // failure: return None, don't advance
    })
}
```
The `Err(_)` arm discards the error message and returns `Ok`. This is what makes `opt` always succeed. The input position is reset to `input` (not `rest`) — we didn't consume anything on failure.

**`with_default` — fallback value instead of `None`:**
```rust
fn with_default<'a, T: Clone + 'a>(default: T, parser: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok(result) => Ok(result),
        Err(_)     => Ok((default.clone(), input)),  // use default, reset position
    })
}
```
`T: Clone` is required because `default` may be returned multiple times (once per parse attempt), and we need to clone it each time.

**`peek` — lookahead without consuming:**
```rust
fn peek<'a, T: Clone + 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, _)) => Ok((Some(value), input)),  // succeeded but don't advance — return original input
        Err(_)         => Ok((None, input)),
    })
}
```
The only difference from `opt`: on success, we return `input` (original position) instead of `rest`. The value is available but the cursor didn't move.

**Usage:**
```rust
// Parsing optional sign prefix
let p = opt(satisfy(|c| c == '+' || c == '-', "sign"));
println!("{:?}", p("+42")); // Ok((Some('+'), "42"))
println!("{:?}", p("42"));  // Ok((None, "42")) — no sign, still Ok, position unchanged

// Default sign is '+' if absent
let p = with_default('+', satisfy(|c| c == '+' || c == '-', "sign"));
println!("{:?}", p("-5")); // Ok(('-', "5"))
println!("{:?}", p("5"));  // Ok(('+', "5")) — defaulted to '+'

// Peek ahead without consuming
let p = peek(satisfy(|c| c.is_ascii_digit(), "digit"));
println!("{:?}", p("123")); // Ok((Some('1'), "123")) — '1' not consumed!
println!("{:?}", p("abc")); // Ok((None, "abc"))
```

## What This Unlocks

- **Optional grammar elements** — sign prefixes, trailing commas, optional type annotations — all expressible cleanly with `opt`.
- **Lookahead decisions** — `peek` lets you decide which parser to apply next based on what's coming, without committing to consume it.
- **The integer parser** (example 161) uses `opt(satisfy(|c| c == '+' || c == '-', "sign"))` as its first step.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Option type | `'a option` (`Some v` / `None`) | `Option<T>` (`Some(v)` / `None`) |
| Always succeeds | Yes — `opt` converts `Error` to `Ok (None, input)` | Same — `Err(_)` becomes `Ok((None, input))` |
| Default values | Any value, no constraint | `T: Clone` required (default may be returned many times) |
| Peek pattern | Non-consuming match | Same: return original `input`, not `rest` |
| Position reset | Immutable strings — automatic | Explicit: discard `rest`, return `input` |
