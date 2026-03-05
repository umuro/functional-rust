📖 **[View on hightechmind.io →](https://hightechmind.io/rust/162-identifier-parser)**

---

# 162: Identifier Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

Parse programming language identifiers — and discover Rust's unique zero-copy `&str` trick along the way.

## The Problem This Solves

Parsing identifiers is the first thing any language parser does. Before you can parse `let x = 42`, you need to recognize that `x` is an identifier (starts with a letter or `_`, continues with letters, digits, or `_`) and that `let` is a keyword (same syntax, but reserved).

This example brings together everything from the series — `satisfy`, `many0`, `map`, and a new Rust-specific technique: returning `&str` directly instead of a `String`. In most parsers, the identifier is returned as an owned `String`. But Rust lets you return a *slice of the original input* — a `&'a str` that borrows from the input without any allocation. This is genuinely unique to Rust's ownership model.

## The Intuition

An identifier rule is: start with `[a-zA-Z_]`, continue with zero or more `[a-zA-Z0-9_]`. That's `satisfy(start_pred)` followed by `many0(satisfy(continue_pred))`. Collect the results into a string, and you have an identifier parser.

The zero-copy version is more interesting. Instead of building a `String` character by character, it scans the input and asks: "How many bytes from the start of this `&str` are valid identifier characters?" Then it returns `&input[..end]` — a slice into the original string that shares its memory. Zero allocations.

Reserved word filtering is a semantic layer on top. Syntactically, `let` is a valid identifier. Semantically, it's not — your language reserves it. You parse the identifier first, then check: if it's in the reserved list, reject it with an error.

## How It Works in Rust

**Approach 1 — allocating with `String`:**
```rust
fn identifier<'a>() -> Parser<'a, String> {
    Box::new(|input: &'a str| {
        // First char: letter or underscore
        let start = satisfy(|c| c.is_ascii_alphabetic() || c == '_', "letter or _");
        let (first, rest) = start(input)?;

        // Remaining chars: letter, digit, or underscore
        let cont = many0(satisfy(|c| c.is_ascii_alphanumeric() || c == '_', "ident char"));
        let (chars, rem) = cont(rest)?;

        // Build the String by collecting
        let mut s = String::with_capacity(1 + chars.len());
        s.push(first);
        for c in chars { s.push(c); }
        Ok((s, rem))
    })
}
// identifier()("hello world") = Ok(("hello", " world"))
// identifier()("_foo bar")    = Ok(("_foo", " bar"))
// identifier()("x1y2z3!")     = Ok(("x1y2z3", "!"))
// identifier()("123")         = Err — can't start with digit
```

**Approach 2 — zero-copy with `&str` slice:**
```rust
fn identifier_slice<'a>() -> Parser<'a, &'a str> {
    Box::new(|input: &'a str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut end = c.len_utf8();  // start after first char
                for ch in chars {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        end += ch.len_utf8();  // accumulate byte length
                    } else {
                        break;
                    }
                }
                // Return a slice of the original input — zero allocation
                Ok((&input[..end], &input[end..]))
            }
            _ => Err("Expected identifier".to_string()),
        }
    })
}
// identifier_slice()("myVar = 5") = Ok(("myVar", " = 5"))
// The returned "myVar" is NOT a new String — it's a pointer into the original input
```
The `end` variable accumulates the byte length of the identifier. `&input[..end]` is a zero-copy reference to that prefix. This is idiomatic Rust and not possible in garbage-collected languages.

**Approach 3 — reserved word rejection:**
```rust
fn identifier_not_reserved<'a>(reserved: &[&str]) -> Parser<'a, String> {
    let reserved: Vec<String> = reserved.iter().map(|s| s.to_string()).collect();
    Box::new(move |input: &'a str| {
        let (name, rest) = identifier()(input)?;  // parse syntactically valid identifier
        if reserved.iter().any(|r| r == &name) {
            Err(format!("'{}' is a reserved word", name))  // semantic rejection
        } else {
            Ok((name, rest))
        }
    })
}
// p("myVar") = Ok(("myVar", ""))
// p("let")   = Err("'let' is a reserved word")
```

## What This Unlocks

- **Language frontend parsing** — identifiers are the most common token in any language parser; this is the production-ready version.
- **Zero-copy tokenization** — `identifier_slice` returns views into the original source, making it practical to tokenize large files without heap pressure.
- **Keyword discrimination** — combining syntactic parsing with reserved-word filtering separates the lexer's job (what *can* be an identifier) from the parser's job (what *should* be one).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String building | `String.make 1 c ^ String.init n (fun i -> ...)` | `String::new()` + `push` |
| Zero-copy output | Not idiomatic (strings are GC values) | `&'a str` — a slice into the original input, no allocation |
| Reserved word check | `List.mem name reserved` | `reserved.iter().any(\|r\| r == &name)` |
| Char classification | Manual: `c >= 'a' && c <= 'z' \|\| ...` | Built-in: `is_ascii_alphabetic()`, `is_ascii_alphanumeric()` |
| Byte advancement | `String.length c` (always 1 in OCaml bytes) | `c.len_utf8()` — correct for multi-byte Unicode |
