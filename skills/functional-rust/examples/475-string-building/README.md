# 475: String Building Patterns

**Difficulty:** 1  **Level:** Beginner

Build strings incrementally and efficiently — the right tool for each situation.

## The Problem This Solves

In Python, building a string with `+=` in a loop is discouraged because each `+=` creates a new string object — O(n²) copies. Instead you collect into a list and call `"".join(parts)`. JavaScript has the same issue; the fix is `Array.prototype.join` or template literals.

Rust has the same concern, but with more explicit control. `String` is a growable heap buffer. You can append to it with `.push_str()` and `.push()`. You can pre-allocate with `String::with_capacity()` to avoid reallocations. And since strings are just iterators of chars, you can collect directly from an iterator — which is often the most idiomatic Rust approach.

Knowing which tool to use matters for both performance and readability. `format!()` is clear but allocates. `push_str` + `with_capacity` is fastest for large builds. `join` is perfect for slice-of-strings. `collect()` is idiomatic when you're already working with an iterator.

## The Intuition

Think of `String` as `Vec<char>` (roughly). You can push to the end cheaply when there's capacity. `String::with_capacity(n)` pre-allocates `n` bytes on the heap — like Python's `bytearray` with a pre-set size. Then `.push_str()` appends without reallocating, as long as you stay within capacity.

OCaml's equivalent is `Buffer.create` + `Buffer.add_string` — Rust's `String::with_capacity` + `push_str` is the same pattern. For joining a list of strings, both languages have a direct equivalent: OCaml's `String.concat sep list` = Rust's `slice.join(sep)`.

For iterator-based building: any iterator that yields `char` or `&str` can be `.collect::<String>()`. This is Rust's most idiomatic approach — transform with iterator adapters, collect at the end.

## How It Works in Rust

```rust
// push_str / push — append to a String
let mut s = String::new();
s.push_str("Hello");   // append a &str
s.push(',');           // append a single char
s.push(' ');
s.push_str("World!");

// with_capacity — avoid reallocations for known-size builds
let mut buf = String::with_capacity(64);
for i in 0..5 {
    buf.push_str(&i.to_string());
    if i < 4 { buf.push(','); }
}
// buf = "0,1,2,3,4", no reallocations if total < 64 bytes

// join — cleanest for &[&str] or &[String]
let words = ["the", "quick", "brown", "fox"];
let sentence = words.join(" ");  // "the quick brown fox"

// collect from iterator — most idiomatic
let upper: String = "hello"
    .chars()
    .map(|c| c.to_ascii_uppercase())
    .collect();  // "HELLO"

// repeat
let divider = "─".repeat(40);

// Combine with iterator: produce Vec<String>, then join
let csv: String = (1..=5)
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(", ");  // "1, 2, 3, 4, 5"
```

## What This Unlocks

- **Log line assembly** — pre-allocate with `with_capacity`, append fields with `push_str`, zero excess allocation.
- **Template rendering** — collect from iterators with `map` + `collect`, transforming as you go.
- **CSV/DSV output** — `.join(",")` on a slice of strings is a one-liner.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable string builder | `Buffer.create n` | `String::with_capacity(n)` |
| Append string | `Buffer.add_string buf s` | `s.push_str(other)` |
| Append char | `Buffer.add_char buf c` | `s.push(c)` |
| Extract result | `Buffer.contents buf` | `buf` (already a `String`) |
| Join with separator | `String.concat sep list` | `slice.join(sep)` |
| Build from iterator | `String.concat "" list` | `.collect::<String>()` |
| Repeat string | `String.concat "" (List.init n ...)` | `"x".repeat(n)` |
