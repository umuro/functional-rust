# 556: Rental / Self-Referential Pattern

**Difficulty:** 4  **Level:** Intermediate-Advanced

Store parsed tokens alongside their source string — without self-referential pointers — using byte-span indices or shared ownership.

## The Problem This Solves

Parsers face a tension: they want to return borrowed `&str` slices pointing into the input buffer (zero-copy), but they also want to *own* that input buffer so the caller doesn't need to keep the original alive. This is the classic "self-referential struct" problem — a struct that holds both data and a reference into that same data.

The `rental` crate attempted to solve this with macros. The `ouroboros` crate does it more safely with proc-macros. But both add complexity and compile-time overhead. In most real code, the cleanest solution is the one demonstrated here: **store byte spans instead of `&str` slices**, reconstruct slices on demand.

The `Arc<String>` alternative shown in `SharedDocument` is useful when you need to share the source across threads or return owned values cheaply — but it trades zero-copy for the overhead of reference-counted cloning.

## The Intuition

The borrow checker forbids storing a `&str` that points into a field of the same struct — the struct would need to borrow from itself before it's finished being constructed. Byte indices solve this: `(usize, usize)` pairs are plain data with no lifetime, and `&self.source[s..e]` reconstructs the view at call time with the correct lifetime.

Think of `ParsedDocument` as a database: the `source` field is the backing store, and `token_spans` is an index. Queries into the index (`get_token`, `tokens()`) produce `&str` results borrowed from `self`, not stored in `self`.

## How It Works in Rust

**Span-indexed document** — parse stores `(start, end)` pairs:
```rust
struct ParsedDocument {
    source: String,
    token_spans: Vec<(usize, usize)>,
}

impl ParsedDocument {
    fn tokens(&self) -> impl Iterator<Item = &str> {
        self.token_spans.iter().map(|&(s, e)| &self.source[s..e])
    }
    fn get_token(&self, i: usize) -> Option<&str> {
        self.token_spans.get(i).map(|&(s, e)| &self.source[s..e])
    }
}
```
The returned `&str` values borrow from `self.source` through `&self` — the compiler sees this as a normal field borrow. No unsafe, no macros.

**Building the index during parsing:**
```rust
for (i, b) in source.bytes().enumerate() {
    let is_space = b == b' ' || b == b'\n' || b == b'\t';
    if !is_space && !in_word { word_start = i; in_word = true; }
    else if is_space && in_word { token_spans.push((word_start, i)); in_word = false; }
}
if in_word { token_spans.push((word_start, source.len())); }
```

**`Arc<String>` alternative** — when tokens need to outlive the parser:
```rust
struct SharedDocument {
    source: Arc<String>,
    tokens: Vec<Arc<String>>,  // cloned substrings, reference-counted
}
```
`Arc::clone` is cheap (atomic increment), but each token is an allocation. Use when you need `Send + Sync` or need tokens to outlive `self`.

**When to use `ouroboros`** — if you genuinely need `&str` fields (e.g., for a zero-copy `serde` deserializer that borrows from a parsed buffer), `ouroboros` generates safe self-referential structs with proc-macro magic. Reach for it only when index-based approaches don't fit.

## What This Unlocks

- **Zero-allocation parsing** — parse a 100MB log file and serve token slices without copying any strings.
- **Lifetime-safe APIs** — callers get `&str` with a lifetime tied to the document, preventing use-after-free.
- **Arc-based sharing** — distribute parsed tokens across threads or into async tasks without lifetime constraints.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Self-referential struct | Natural (GC, everything is a ref) | Forbidden without `Pin`/`unsafe`/`ouroboros` |
| Zero-copy sub-string | `Bytes.sub` (O(1) but lib-specific) | `&source[s..e]` — zero-copy, lifetime-checked |
| Shared ownership | Natural (GC shares freely) | `Arc<T>` — reference counted, `clone` is cheap |
| Span indexing pattern | Less necessary (no borrow checker) | Idiomatic workaround for self-referential structs |
