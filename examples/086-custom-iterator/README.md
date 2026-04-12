[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 086 — Custom Iterator with State
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement three stateful custom iterators: a `Counter` that steps by a given value indefinitely, a `Fib` Fibonacci iterator, and a `Collatz` iterator that terminates at 1. Demonstrate that implementing `next` unlocks the full iterator adapter chain. Compare with OCaml's closure-based counters and `Seq` lazy sequences.

## Learning Outcomes

- Hold mutable state in struct fields rather than closures for complex iterators
- Return `Some(val)` forever from an infinite iterator; use `.take(n)` to bound it
- Implement a finite iterator by tracking a `done_` flag and returning `None`
- Use the Collatz sequence as a real-world example of a self-terminating iterator
- Map Rust's mutable struct iterator to OCaml's `mutable ref` closure and `Seq` thunk
- Compose multiple custom iterators using `.zip`, `.map`, `.sum`

## Rust Application

`Counter` stores `current` and `step`, advancing `current` by `step` on each `next` call. `Fib` stores the pair `(a, b)` and updates with `(a + b, a)` — an infinite sequence of Fibonacci numbers. `Collatz` stores the current `n` and a `done_` flag; it terminates when `n` reaches 1 by setting `done_` to `true` after yielding 1. All three implement `type Item = u64` (or `i32`) and the single `next` method — the rest of the adapter chain is inherited.

## OCaml Approach

OCaml uses mutable `ref` cells for stateful counters: `let n = ref (start - step) in fun () -> n := !n + step; Some !n`. Fibonacci uses the `Seq` module: `let rec aux a b () = Seq.Cons(a, aux b (a+b))`. The `take_seq` helper materialises the first `n` elements. Collatz is also expressed as a `Seq` with termination when `n = 1`. OCaml's approach requires explicit `Seq.Cons`/`Seq.Nil` construction while Rust's `Option<T>` return is simpler.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| State storage | Struct fields | Mutable `ref` or closure capture |
| Infinite | Return `Some(…)` always | `Seq.Cons(…, thunk)` always |
| Finite | Return `None` at end | `Seq.Nil` at end |
| Lazy | Pull-based (call `next`) | Thunk-based (call `seq ()`) |
| Adapter chain | Free from `Iterator` trait | Manual `seq_map`/`seq_filter` |
| Termination signal | `done_` flag or pattern on value | `Seq.Nil` |

The Collatz iterator demonstrates a common pattern: an iterator that terminates when some condition on the internal state is met. This is cleaner than computing the full sequence upfront and is memory-efficient for long sequences.

## Exercises

1. Implement a `Prime` iterator that produces prime numbers infinitely using a sieve stored in a `HashSet`.
2. Add `DoubleEndedIterator` for `Counter` (since it has a known step, going backwards is well-defined).
3. Write a `Zip3<A, B, C>` iterator that zips three iterators into triples.
4. Implement a `RunLength<I: Iterator>` adapter that wraps any iterator and emits `(count, value)` pairs.
5. In OCaml, implement a lazy Sieve of Eratosthenes using `Seq` that streams primes without storing the full sieve.
