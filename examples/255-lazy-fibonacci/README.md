📖 **[View on hightechmind.io →](https://hightechmind.io/rust/255-lazy-fibonacci)**

---

# Example 255: Lazy Fibonacci

**Difficulty:** ⭐⭐
**Category:** Lazy / Infinite Sequences
**OCaml Source:** https://cs3110.github.io/textbook/chapters/ds/streams.html

## Problem Statement

Generate an infinite stream of Fibonacci numbers and take the first `n` of them,
without computing (or storing) the whole sequence up front.

## Learning Outcomes

- How Rust's `Iterator` trait models lazy, potentially-infinite sequences
- How to encode OCaml's thunk-based `stream` type using `Box<dyn Fn()>`
- Why recursive types require heap indirection (`Box`) in Rust
- How ownership interacts with closures that capture state (`move` closures)

## OCaml Approach

OCaml defines a coinductive `'a stream = Cons of 'a * (unit -> 'a stream)`.
The tail is a *thunk* — a suspended computation `fun () -> ...` — evaluated only
when needed.  `fibs a b = Cons (a, fun () -> fibs b (a+b))` creates an infinite
structure; `take n` forces exactly `n` steps, never evaluating the rest.

## Rust Approach

**Idiomatic Rust** uses the `Iterator` trait.  A `FibIter` struct holds only two
`u64` values and advances them in `next()`.  The standard library's `.take(n)`
and `.collect()` adapters replace OCaml's `take` function.  No heap allocation is
needed and the compiler can often unroll or vectorise the loop.

**Thunk-based Rust** mirrors OCaml directly: `Stream<T>` holds a `head` and a
`Box<dyn Fn() -> Stream<T>>` tail.  `Box` is required because `Stream` contains
itself — without indirection the type would have infinite size.  A `move` closure
captures `a` and `b` by value, keeping each tail self-contained.

## Key Differences

1. **Lazy abstraction:** OCaml uses a bespoke `stream` type; Rust uses the
   built-in `Iterator` trait that the entire standard library understands.
2. **Heap indirection:** OCaml's GC handles recursive types transparently; Rust
   requires an explicit `Box` for any recursive type to bound its stack size.
3. **Closure capture:** OCaml closures capture by reference (GC-managed); Rust
   `move` closures take ownership, making each thunk independent and `'static`.
4. **Infinite safety:** Both languages are safe with infinite streams so long as
   you only force finite prefixes; neither will diverge on `take n`.
