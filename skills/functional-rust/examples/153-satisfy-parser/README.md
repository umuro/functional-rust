# 153: Satisfy Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`satisfy(pred)` accepts any character where a predicate returns `true` — the most powerful atomic parser, from which all character-class parsers are built.

## The Problem This Solves

Example 152 showed `char_parser`, `any_char`, `none_of`, and `one_of`. But what if you want "any digit"? You'd need to either list all ten digit characters in `one_of`, or write a new function from scratch. That's not composable — every new character class means a new function.

`satisfy` solves this by abstracting the condition. Instead of hardcoding what the character must be, you pass a predicate: any function `char -> bool`. "Is it a digit?" is just `|c| c.is_ascii_digit()`. "Is it a hex digit?" is `|c| c.is_ascii_hexdigit()`. "Is it a lowercase letter or a dash?" is `|c| c.is_ascii_lowercase() || c == '-'`. You never need to write a new atomic parser — just pass a different predicate.

This is the essence of higher-order programming: instead of writing many specialized functions, you write one general function and parameterize the varying part.

## The Intuition

You already use predicates all the time. `Vec::retain(|x| x > 0)` keeps only positive elements. `Iterator::filter(|s| s.starts_with("http"))` keeps matching strings. `satisfy` is the same idea applied to character parsing: keep the character if the predicate says yes, reject it otherwise.

Rust's `char` type has rich built-in predicates:
- `c.is_ascii_digit()` — `'0'`..`'9'`
- `c.is_ascii_alphabetic()` — `'a'`..`'z'` and `'A'`..`'Z'`
- `c.is_ascii_alphanumeric()` — both of the above
- `c.is_ascii_whitespace()` — space, tab, newline, carriage return
- `c.is_ascii_uppercase()` / `c.is_ascii_lowercase()`

These are the building blocks. `satisfy` is the glue.

## How It Works in Rust

**The `satisfy` combinator:**
```rust
fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,  // pred must be a function from char to bool,
                                // and must live at least as long as 'a
{
    let desc = desc.to_string();  // own the description for the closure
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),  // predicate passes
            Some(c) => Err(format!("'{}' does not satisfy {}", c, desc)),
            None    => Err(format!("Expected {}, got EOF", desc)),
        }
    })
}
```
The `where F: Fn(char) -> bool + 'a` bound means: any callable that takes a `char` and returns `bool`, as long as it doesn't hold references that outlive `'a`. In practice, closures with no captures satisfy this trivially. The `desc` string is for error messages — "digit", "letter", etc.

**Building specific parsers from `satisfy`:**
```rust
fn is_digit<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_digit(), "digit")
}

fn is_letter<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_alphabetic(), "letter")
}

// Custom predicate inline — no new function needed:
let hex = satisfy(|c| c.is_ascii_hexdigit(), "hex digit");
let sign = satisfy(|c| c == '+' || c == '-', "sign");
let vowel = satisfy(|c| "aeiou".contains(c), "vowel");
```

**`satisfy_or` for richer error messages:**
```rust
fn satisfy_or<'a, F, E>(pred: F, on_fail: E) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
    E: Fn(char) -> String + 'a,  // error message depends on what was found
{
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(on_fail(c)),  // custom error using the actual character
            None    => Err("Unexpected EOF".to_string()),
        }
    })
}
```

## What This Unlocks

- **Every character class parser you'll ever need** — digits, letters, whitespace, hex, punctuation — all expressible in one line with `satisfy`.
- **Custom parsers for domain-specific needs** — "valid URL character", "valid identifier character", "printable ASCII" — just write the predicate.
- **The foundation of examples 155–162** — `many0(satisfy(...))`, `map(satisfy(...), ...)`, and every parser combinator in the series uses `satisfy` internally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Predicate type | `char -> bool` (inferred) | `F: Fn(char) -> bool + 'a` (explicit bound) |
| Char classification | Manual: `c >= '0' && c <= '9'` | Built-in: `c.is_ascii_digit()` |
| Closure syntax | `fun c -> c >= '0' && c <= '9'` | `\|c\| c.is_ascii_digit()` |
| Type annotations | None (inference handles it) | Bound on type parameter `F` needed |
| Custom errors | Description string | Can be a closure `Fn(char) -> String` |
