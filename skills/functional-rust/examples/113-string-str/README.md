# 113: String vs &str

**Difficulty:** 2  **Level:** Intermediate

`String` is an owned, heap-allocated string you can grow; `&str` is a borrowed view into any string data — use `&str` in function parameters to accept both without allocating.

## The Problem This Solves

In C, strings are `char*` — a raw pointer. Owned, borrowed, literal, stack-allocated — the type doesn't tell you. You discover which it is when you try to `free()` the wrong one and the program crashes, or when you modify a string literal and get a segfault.

In Java, `String` is always heap-allocated and immutable. Simple — but you can't have a "view into part of a string" without copying. Every substring allocates. String-heavy code in Java has serious GC pressure.

OCaml's `string` type is similarly a single concept: immutable, GC-managed. No distinction between owned and borrowed. Clean, but at the cost of GC overhead on all strings.

Rust has two types because they solve different problems. `String` is owned heap-allocated data — you can push, truncate, and pass it around while keeping ownership clear. `&str` is a borrowed slice — a pointer and a length pointing into any string data (a `String`, a string literal baked into the binary, or a substring). No allocation. Pass `&str` to functions and you never force the caller to allocate.

## The Intuition

`String` *owns* the text on the heap; `&str` *borrows* a view into any existing text — using `&str` in function signatures lets callers pass string literals, slices, or `String` references without forcing an allocation.

## How It Works in Rust

```rust
// String: owned, heap-allocated, growable
let mut owned: String = String::from("hello");
owned.push_str(", world"); // grows in-place
owned.push('!');
println!("{}", owned); // "hello, world!"

// &str: borrowed view — no allocation
let borrowed: &str = "hello, world!"; // points into binary (static lifetime)
let slice: &str = &owned[0..5];       // points into the String

// Function that accepts both &str and &String
// BAD: fn greet(name: &String) — forces callers to have a String
// GOOD: fn greet(name: &str) — works with literals AND String
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn demo() {
    let owned = String::from("Alice");
    greet(&owned);        // coerces &String to &str automatically
    greet("Bob");         // string literal is already &'static str
    greet(&owned[0..3]);  // slice of String — also &str
}

// Converting between them
let s: String = "hello".to_string();    // &str → String (allocates)
let s: String = String::from("hello");  // same
let r: &str = &s;                       // String → &str (no allocation)
let r: &str = s.as_str();               // explicit

// String owns its buffer; dropping String frees the memory
// &str borrows — it can't outlive the data it points to
fn returns_str() -> &'static str {
    "static lifetime — baked into binary"
}

// This would be a compile error:
// fn returns_local_str() -> &str {
//     let s = String::from("temporary");
//     &s // ERROR: s is dropped, &s would dangle
// }
```

## What This Unlocks

- **Zero-copy function APIs** — `fn process(s: &str)` accepts string literals, borrowed `String`s, and substrings without any caller-side allocation.
- **Substring views** — `&s[start..end]` gives a zero-copy view into any part of a string; essential for parsers and text processing.
- **Clarity about ownership** — `String` in a struct means "I own this text." `&'a str` in a struct means "I borrow this text from somewhere that must outlive me."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String type | Single `string` (immutable, GC) | `String` (owned) and `&str` (borrowed) |
| Substring | `String.sub` — always copies | `&s[start..end]` — zero-copy view |
| String literal type | `string` | `&'static str` |
| Function parameter | `string` — always owned ref | Prefer `&str` — accepts all string kinds |
| Mutability | Immutable (use `Bytes` for mutable) | `String` is mutable; `&str` is read-only |
