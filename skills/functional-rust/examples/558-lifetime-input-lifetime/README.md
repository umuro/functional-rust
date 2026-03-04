# 558: Input Lifetimes Guide

**Difficulty:** 4  **Level:** Intermediate-Advanced

Know when lifetime annotations on function inputs are required, when elision handles them, and how to read the elision rules confidently.

## The Problem This Solves

Most Rust learners memorize "add `'a` when the compiler complains" without understanding the underlying rules. This leads to over-annotation (adding lifetimes everywhere, making signatures noisy), under-annotation (guessing wrong, getting subtle bugs), and confusion when the compiler error isn't clear.

The three elision rules cover the vast majority of function signatures. The remaining cases — multiple input references where the output borrows from a *specific* one — require explicit annotation. Knowing which rule applies to your function is the skill that separates a Rust beginner from an intermediate.

## The Intuition

Lifetime annotations on inputs answer one question: "which input does the output borrow from?" When there's only one input reference, the answer is obvious and elision states it for you. When there's `&self` and the output comes from `self`, rule 3 applies. Only when you have multiple input references *and* the output borrows from a specific non-self one do you need to write `'a` explicitly.

Think of `'a` as a tag: you're tagging input and output with the same label to say "these are connected." The compiler uses these tags to verify the borrowing is safe.

## How It Works in Rust

**Rule 1 — each input gets its own anonymous lifetime (implicit):**
```rust
fn first_element(slice: &[i32]) -> Option<&i32> {
    slice.first()
}
// Desugars to: fn first_element<'a>(slice: &'a [i32]) -> Option<&'a i32>
// Only one input ref → output borrows from it. Elision works.
```

**Rule 3 — `&self` method output borrows from self:**
```rust
impl Buffer {
    fn as_bytes(&self) -> &[u8] { &self.data }
    // Desugars to: fn as_bytes<'a>(&'a self) -> &'a [u8]
}
```
When `&self` is one of the inputs, the output is assumed to borrow from `self` unless you say otherwise.

**Multiple inputs — must annotate when output borrows from a specific one:**
```rust
fn get_key<'a, 'b>(map_key: &'a str, _context: &'b str) -> &'a str {
    map_key  // output borrows from 'a, NOT 'b
}
```
Without `'a`, the compiler can't tell which input the output borrows from. Here, `_context` can be dropped while the return value lives — the distinct lifetimes express that.

**Two outputs from the same input — single lifetime covers both:**
```rust
fn split_around<'a>(haystack: &'a str, needle: char) -> (&'a str, &'a str) {
    match haystack.find(needle) {
        Some(i) => (&haystack[..i], &haystack[i+1..]),
        None    => (haystack, ""),
    }
}
```
Both halves borrow from `haystack`, so they get the same `'a`.

**When the output doesn't borrow at all — no annotation needed:**
```rust
fn apply_to_str<F: Fn(&str) -> usize>(s: &str, f: F) -> usize {
    f(s)  // output is usize — no borrow, no annotation needed
}
```

## What This Unlocks

- **Reading compiler errors confidently** — when the compiler asks for a lifetime, you know exactly why and which annotation to add.
- **Clean API design** — annotate exactly what's needed, not more. Noisy lifetime signatures are a code smell.
- **Drop precision** — distinguishing `'a` and `'b` lets callers drop one input early while continuing to use the output that borrows from the other.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference validity | GC tracks all refs | Compile-time lifetime analysis |
| Single-input function | No annotation needed | Elision rule 2: output gets same lifetime as input |
| Method returning field | Natural | Elision rule 3: output borrows from `&self` |
| Multiple input refs | No annotation needed | Must annotate when output borrows from a specific one |
