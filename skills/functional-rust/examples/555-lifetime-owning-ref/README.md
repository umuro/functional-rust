# 555: Owning References Pattern

**Difficulty:** 4  **Level:** Intermediate-Advanced

Build a struct that owns its data and simultaneously provides borrowed views into it — without self-referential pointers.

## The Problem This Solves

A common pattern in parsing, indexing, and query engines is: you load data once, then hand out many cheap views into it. In languages with GC, this is trivial — everything is a reference. In Rust, the naive approach (store both the `Vec` and a `&[T]` pointing into it) is self-referential and rejected by the borrow checker.

The solution is to separate *data* from *metadata*. The struct owns the data (e.g. a `Vec<u8>`), and views are expressed as indices or byte-offsets rather than raw references. When you need a `&[u8]`, you reconstruct it from the stored indices at call time. The lifetime of the returned reference is tied to `&self`, not stored inside `self` — and that is exactly what the borrow checker needs to see.

This pattern appears in `ParsedDocument`, query builders, arena allocators, and any cache that serves sub-slices of a larger buffer.

## The Intuition

Instead of storing `&str` slices (which would require the struct to borrow from itself), store `(start, end)` byte pairs. When the caller asks for a token, compute `&self.source[start..end]` on the fly. The returned `&str` borrows from `self`, so its lifetime is tied to the struct's lifetime — no self-referential pointer needed.

For query builder patterns, store the source data in the struct and return iterators or references with lifetime `'a` tied to `&'a self`. The caller can consume results as long as the builder lives.

## How It Works in Rust

**Index-based view** — store positions, not pointers:
```rust
struct ParsedDocument {
    source: String,
    token_spans: Vec<(usize, usize)>,  // (start, end) byte positions
}

impl ParsedDocument {
    fn get_token(&self, i: usize) -> Option<&str> {
        self.token_spans.get(i).map(|&(s, e)| &self.source[s..e])
    }
}
```
`&self.source[s..e]` has lifetime `'self` — perfectly fine.

**Iterator returning borrowed data** — the `'a` on `&'a self` flows into the `impl Iterator`:
```rust
impl QueryBuilder {
    fn filter<'a>(&'a self, pred: impl Fn(&i32) -> bool)
        -> impl Iterator<Item = &'a i32>
    {
        self.source.iter().filter(move |&&ref x| pred(x))
    }
}
```
The iterator holds a reference to `self.source`, so it can't outlive the builder.

**Owned + metadata tuple** — return ownership and computed metadata together:
```rust
fn process_and_view(data: Vec<String>) -> (Vec<String>, Vec<usize>) {
    let lengths: Vec<usize> = data.iter().map(|s| s.len()).collect();
    (data, lengths)
}
```
Compute what you need from `&data`, then move `data` into the return tuple. No lifetime juggling needed.

## What This Unlocks

- **Zero-copy document parsing** — parse once, serve many `&str` tokens without extra allocation.
- **Query builder APIs** — fluent interfaces that borrow from internal data and return typed iterators.
- **Avoiding `Arc<String>` overhead** — when you own the data, index-based views are cheaper than cloning into `Arc`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sub-string view | `String.sub` (copies) or `Bytes.sub_string` | `&s[start..end]` (zero-copy borrow) |
| Self-referential struct | Not a problem (GC) | Forbidden; use indices instead |
| Returning borrowed iterator | GC handles refs | `impl Iterator<Item = &'a T>` with explicit lifetime |
| Owned + borrowed tuple | Natural (everything is reference) | Return `(Vec<T>, Vec<usize>)` — compute metadata before moving |
