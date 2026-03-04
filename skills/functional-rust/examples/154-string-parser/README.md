# 154: String Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`tag("hello")` matches an exact string literal — the first multi-character parser, and a zero-copy one at that.

## The Problem This Solves

Character parsers work one rune at a time. But real grammars need to match keywords: `"true"`, `"false"`, `"null"`, `"fn"`, `"let"`. You could chain five `char_parser` calls, but that's clunky. You want a single combinator that says "match this whole word."

Beyond keywords, string parsers are the first place where Rust's zero-copy model pays off. When you match `"hello"` in a string, you don't need to allocate a new `String` — you can return a `&str` that points directly into the original input. For a parser that might match millions of tokens, this avoids millions of allocations.

## The Intuition

`tag("hello")` asks: "Does the input *start with* `hello`?" If yes, return a slice of the input that covers exactly those five bytes, and set the remaining input to start at byte 5. If no, report the mismatch.

The key is `starts_with` — a single O(n) check that handles all the character comparison at once. No loops, no intermediate strings.

Case-insensitive matching (`tag_no_case`) is the same idea, but it compares lowercased versions. The returned slice is still from the original input — you tell the caller "I matched *something* of the right length here" even if the casing differs.

## How It Works in Rust

**Exact match with `tag`:**
```rust
fn tag<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_owned = expected.to_string();  // own the string; closure will capture it
    Box::new(move |input: &'a str| {
        if input.starts_with(&expected_owned) {
            // Zero-copy: slice into the original input
            let matched   = &input[..expected_owned.len()];  // the matched portion
            let remaining = &input[expected_owned.len()..];  // what comes after
            Ok((matched, remaining))
        } else {
            Err(format!("Expected \"{}\"", expected_owned))
        }
    })
}
```
The return type `&'a str` means "a slice borrowed from the input `'a`" — no allocation. Both `matched` and `remaining` are views into the same original string.

**Case-insensitive with `tag_no_case`:**
```rust
fn tag_no_case<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_lower = expected.to_lowercase();  // allocate once at parser creation
    let len = expected.len();
    Box::new(move |input: &'a str| {
        if input.len() >= len && input[..len].to_lowercase() == expected_lower {
            Ok((&input[..len], &input[len..]))  // return original casing from input
        } else {
            Err(format!("Expected \"{}\" (case insensitive)", expected_lower))
        }
    })
}
```
Note: `input[..len].to_lowercase()` allocates a temporary `String` for comparison, but the *returned value* is still a zero-copy slice.

**Character-by-character matching (shows the composition):**
```rust
fn string_from_chars<'a>(expected: &str) -> Parser<'a, String> {
    let expected = expected.to_string();
    Box::new(move |input: &'a str| {
        let mut remaining = input;
        for expected_char in expected.chars() {
            match remaining.chars().next() {
                Some(c) if c == expected_char => {
                    remaining = &remaining[c.len_utf8()..];  // advance one char at a time
                }
                Some(c) => return Err(format!("Expected '{}', got '{}'", expected_char, c)),
                None    => return Err(format!("Expected '{}', got EOF", expected_char)),
            }
        }
        Ok((expected.clone(), remaining))
    })
}
```
This version returns an owned `String` (allocated) rather than a borrowed `&str`. Less efficient, but shows how `tag` could be built from char parsers.

**Usage:**
```rust
let p = tag("hello");
println!("{:?}", p("hello world")); // Ok(("hello", " world"))
println!("{:?}", p("world"));       // Err("Expected \"hello\"")
println!("{:?}", p("hel"));         // Err — too short

let p = tag_no_case("Hello");
println!("{:?}", p("HELLO!"));  // Ok(("HELLO", "!"))
println!("{:?}", p("HeLLo"));   // Ok(("HeLLo", ""))
```

## What This Unlocks

- **Keyword and delimiter parsers** — match `"true"`, `"false"`, `"null"`, `"->"`, `"::"`, `"=>"` as single parser calls.
- **Zero-cost abstractions** — `tag` returns `&str` slices, not `String` allocations; parsing a 1MB file doesn't mean 1MB of extra allocations.
- **Building block for `choice`** — example 157 uses `choice(vec![tag("true"), tag("false"), tag("null")])` as a direct application.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Return type | `string` (always a new copy) | `&'a str` (zero-copy slice into input) |
| Prefix check | `String.sub input 0 len = expected` | `input.starts_with(expected)` |
| Case conversion | `String.lowercase_ascii s` | `s.to_lowercase()` (allocates) |
| Memory model | GC manages string copies | Explicit lifetimes; slicing borrows original |
| Char-by-char | Recursive with `String.make` | Iterative with `&remaining[c.len_utf8()..]` |
