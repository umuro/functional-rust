# 369: Cow<str> Clone-on-Write

**Difficulty:** 2  **Level:** Intermediate

Accept both owned and borrowed strings — clone only when mutation is needed.

## The Problem This Solves

Functions that process strings face a constant dilemma: take `&str` (fast, but can't return owned data produced inside), or take `String` (flexible, but forces callers to clone). The classic case: a function that usually returns the input unchanged, but occasionally needs to escape special characters or normalize whitespace. Most inputs pass through untouched — why allocate for every call?

`Cow<'a, str>` (Clone-on-Write) solves this. It's an enum that holds either a borrowed `&str` or an owned `String`. You return the borrowed reference when no modification is needed, and produce an owned `String` only when you actually change the content. The caller gets `O(1)` on the fast path, allocation only on the slow path.

This pattern appears throughout the standard library and major crates wherever "maybe-owned" data flows through an API.

## The Intuition

`Cow<'b, B>` is an enum with two variants: `Borrowed(&'b B)` and `Owned(B::Owned)`. For strings, that's `Borrowed(&str)` and `Owned(String)`. You can call `.to_mut()` on it to get a `&mut String` — at that point, if it was borrowed, it clones once and becomes owned. Hence: clone-on-write.

The tradeoff is API clarity vs performance. `Cow<str>` is slightly awkward to work with compared to just `String`. Use it when profiling shows unnecessary allocations on a hot path, or when designing a library API that must be zero-copy for the common case.

## How It Works in Rust

```rust
use std::borrow::Cow;

fn escape_html(input: &str) -> Cow<str> {
    if input.contains('<') || input.contains('>') {
        // Slow path: allocate and modify
        Cow::Owned(input.replace('<', "&lt;").replace('>', "&gt;"))
    } else {
        // Fast path: return borrow, zero allocation
        Cow::Borrowed(input)
    }
}

let clean = escape_html("hello world");   // Borrowed — no alloc
let dirty = escape_html("a < b");         // Owned — one alloc

// Both work transparently as &str
println!("{}", clean.len());
println!("{}", dirty.to_uppercase());

// Force owned when you need a String
let s: String = clean.into_owned();
```

## What This Unlocks

- **Zero-copy processing pipelines** — only allocate when the data actually changes.
- **Library APIs** — accept `impl Into<Cow<str>>` to be flexible for both `&str` and `String` callers.
- **Serialization** — deserialize strings without copying when the input is already valid UTF-8.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrowed vs owned | `string` (immutable, GC-managed) | `&str` (borrowed) vs `String` (owned) |
| Maybe-owned | No equivalent — GC hides it | `Cow<'a, str>` |
| Clone on mutation | Automatic (structural sharing) | Explicit `.to_mut()` triggers clone |
| Generic Cow | N/A | `Cow<'a, B>` where `B: ToOwned` — works for slices too |
