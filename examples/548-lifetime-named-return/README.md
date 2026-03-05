📖 **[View on hightechmind.io →](https://hightechmind.io/rust/548-lifetime-named-return)**

---

# 548: Named Lifetime in Return Types

**Difficulty:** 3  **Level:** Intermediate

Explicitly connect a returned reference's lifetime to the correct input.

## The Problem This Solves

When a function returns a reference, the compiler needs to know how long that reference is valid. Usually it can figure this out via lifetime elision rules — one input parameter means the output borrows from it. But when you have multiple input references, or when you want to make it explicit that the output borrows from *one specific* input and not another, elision doesn't help. Named lifetimes let you declare that explicitly.

This matters in real code. Imagine a function that takes a "data" slice and a "config" string, and returns a subslice of the data. If you drop the config early, the slice should still be valid. Named lifetimes let you say exactly that: the return borrows from `data`, not from `config`. Without this, you either get a compile error or you accidentally constrain both inputs to the same lifetime, making callers keep both alive longer than necessary.

Named lifetimes in struct methods are also essential when you want to return a reference that comes from a method argument rather than from `self` — the compiler defaults to the `self` lifetime under elision rule 3, which may be wrong.

## The Intuition

A named lifetime is a label you attach to references to say "these two things are connected." `fn foo<'a>(x: &'a str, _y: &str) -> &'a str` says: "the output borrows from x; y's lifetime is irrelevant to the output." The compiler enforces that: wherever you call `foo`, the returned reference lives only as long as `x`.

Think of it as a contractual annotation: you're telling both the caller and the compiler the relationship between inputs and outputs. The compiler checks that your body actually satisfies the contract.

## How It Works in Rust

1. **Single input** — elision handles it; `fn max<'a>(slice: &'a [i32]) -> Option<&'a i32>` is identical to the elided version.
2. **Multiple inputs, output from one** — name the relevant lifetime: `fn f<'a>(data: &'a str, _cfg: &str) -> &'a str`; the unrelated parameter is unannotated.
3. **Either input** — when output could come from either, both must share the same lifetime: `fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str`.
4. **Struct methods** — elision rule 3 ties output to `&self`; to return from an argument instead, use distinct lifetimes: `fn find<'doc, 'pat>(&'doc self, pat: &'pat str) -> Option<&'doc str>`.
5. **Generic functions** — `fn get<'a, T>(container: &'a [T], i: usize) -> Option<&'a T>` works the same way; lifetime bounds compose with type bounds.

## What This Unlocks

- Return references to subsets of data without forcing callers to keep unrelated inputs alive.
- Express precise ownership contracts in APIs — callers know exactly what to keep alive.
- Write parsers and views that borrow from a source string without tying the lifetime to auxiliary parameters.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference lifetimes | GC-managed; no annotations needed | Compiler-tracked; explicit when ambiguous |
| Multiple input references | No constraint | Must name lifetime to specify which input the output borrows from |
| Return borrow from self vs arg | N/A | Distinct lifetime names needed to return from a specific argument |
| Lifetime elision | N/A | Three rules; breaks down with multiple reference parameters |
