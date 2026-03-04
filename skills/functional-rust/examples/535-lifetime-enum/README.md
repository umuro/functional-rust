# 535: Lifetimes in Enums

**Difficulty:** 3  **Level:** Intermediate

Enum variants that hold references require the same lifetime treatment as structs — the enum cannot outlive the data it borrows.

## The Problem This Solves

Enums are frequently used in parsers and type-safe protocols. When a variant holds a reference to an input string, without lifetime annotations you get:

```rust
enum Token {
    Word(&str),     // error: missing lifetime specifier
    Number(i64),
    End,
}
```

The compiler needs to know how long `Word`'s `&str` is valid. Without that, nothing stops you from extracting a `Token::Word(s)` and using `s` after the source string is dropped — a classic use-after-free.

Lifetime annotations on enums also enable zero-copy parsers: instead of allocating a `String` for each token, you return slices pointing into the original input. The compiler enforces that those slices don't outlive the input.

## The Intuition

An enum with a lifetime `<'a>` is just a sum type that happens to contain a view. The `'a` parameter applies to any variant that holds a `&'a` reference. Variants without references (like `Number(i64)`) are unaffected — they're owned data and don't participate in the lifetime constraint.

Pattern matching on a lifetime-annotated enum works exactly the same as without lifetimes. The annotation only adds a compile-time check that the enum doesn't outlive its borrowed data.

## How It Works in Rust

**The annotation:**

```rust
// 'a declares: any variant holding a &str borrows from a source that lives for 'a
#[derive(Debug, PartialEq)]
enum Token<'a> {
    Word(&'a str),      // zero-copy slice from input
    Number(i64),        // owned — not constrained by 'a
    Punctuation(char),  // owned — not constrained by 'a
    End,                // no data at all
}
```

**Using it in a parser:**

```rust
fn parse_token(input: &str) -> Token<'_> {
    //                                ^^^ 'lifetime of input flows to Token
    let input = input.trim_start();
    if input.is_empty() { return Token::End; }
    
    let first = input.chars().next().unwrap();
    if first.is_alphabetic() {
        let end = input.find(|c: char| !c.is_alphanumeric())
                       .unwrap_or(input.len());
        Token::Word(&input[..end])  // zero-copy slice — valid as long as input is
    } else {
        Token::Number(42) // placeholder
    }
}
```

**Generic result enum with lifetime:**

```rust
// Both the success value and the remaining input borrow from the source
enum ParseResult<'a, T> {
    Ok(T, &'a str),        // parsed value + remaining input
    Err(&'a str, String),  // failing position (borrowed) + error message (owned)
}
```

**Pattern matching works normally:**

```rust
let input = String::from("hello world");
match parse_token(&input) {
    Token::Word(w)       => println!("word: {}", w),  // w is &'a str
    Token::Number(n)     => println!("num: {}", n),
    Token::Punctuation(c)=> println!("punct: {}", c),
    Token::End           => println!("end"),
}
```

## What This Unlocks

- **Zero-copy lexers** — a full tokenizer can process an entire file and return slices into it without allocating a single `String`. Performance on par with handwritten C.
- **Type-safe protocol decoders** — a `Frame<'a>` enum whose variants borrow from a packet buffer gives compile-time proof the frame doesn't outlive its packet.
- **Mixing owned and borrowed in one type** — `JsonValue<'a>` can hold `Str(&'a str)` for zero-copy string parsing while `Int(i64)` and `Bool(bool)` own their data naturally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Variant with reference | ADT constructors hold any value; GC manages validity | Variants with `&'a str` require `<'a>` on the enum; compiler tracks validity |
| Zero-copy parsing | `Bigarray.t` or manual unsafe needed for zero-copy | `&'a str` variants are naturally zero-copy — slice the input |
| Mixed owned/borrowed variants | GC handles all — no distinction needed | Some variants owned (no 'a constraint), others borrowed (participate in 'a) |
| Result types in parsers | `('a, 'b) result` or custom type | `ParseResult<'a, T>` — remaining input borrows from source, error message owned |
| Enum lifetime scope | N/A | Enum<'a> cannot outlive its 'a — enforced at all use sites |
