📖 **[View on hightechmind.io →](https://hightechmind.io/rust/880-custom-iterator)**

---

# 880-custom-iterator — Custom Iterator

## Problem Statement

Standard ranges and slice iterators cover most use cases, but real programs require custom sequences: Fibonacci numbers, arithmetic progressions with arbitrary step sizes, geometric sequences, or domain-specific data streams. Implementing the `Iterator` trait for a custom struct allows these sequences to integrate fully with Rust's iterator ecosystem — all adapter methods (map, filter, take, chain) work automatically. This example shows two custom iterators: a stateful Fibonacci generator and a generic step range, both demonstrating how minimal trait implementation unlocks a rich API.

## Learning Outcomes

- Implement stateful iterators using struct fields to track position
- Build an infinite iterator (Fibonacci) that always returns `Some`
- Build a finite iterator (StepRange) that returns `None` when exhausted
- Use custom iterators with standard adapter methods like `.take()` and `.filter()`
- Compare with OCaml's `Seq.unfold` for generating custom sequences

## Rust Application

`Fibonacci` stores `(a, b)` and advances with `(b, a+b)` on each `next`, always returning `Some(a)` — an infinite iterator. `StepRange<i64>` stores `(current, end_, step)` and returns `None` when `current >= end_`. Both are usable directly in `for` loops or chained with `.take(10)`, `.filter(|x| x % 2 == 0)`, `.collect()`, and any other iterator method. The generic `StepRange<T>` version demonstrates custom iterators with type parameters and `PartialOrd + AddAssign` bounds.

## OCaml Approach

OCaml generates custom sequences with `Seq.unfold: ('a -> ('b * 'a) option) -> 'a -> 'b Seq.t`. The seed state is passed functionally; each step returns `Some(value, next_state)` or `None`. Fibonacci: `Seq.unfold (fun (a, b) -> Some(a, (b, a+b))) (0, 1)`. Step range: `Seq.unfold (fun i -> if i > n then None else Some(i, i + step)) start`. The functional style avoids mutable struct fields; Rust uses mutable struct state within the `Iterator` implementation.

## Key Differences

1. **Mutable state**: Rust iterators carry mutable state in the struct, modified by `next(&mut self)`; OCaml `Seq.unfold` threads state functionally.
2. **Infinite sequences**: Both support infinite sequences; Rust via `always-Some`, OCaml via `Seq.unfold` with `Some` always.
3. **Integration**: Both integrate fully with their respective ecosystem (adapter chains in Rust, `Seq.map`/`Seq.filter` in OCaml).
4. **Reusability**: Rust iterators are consumed once (stateful); OCaml `Seq` values can be traversed multiple times (immutable).

## Exercises

1. Implement a `Collatz` iterator that generates the Collatz sequence starting from a given number until it reaches 1.
2. Implement a `Geometric<T>` iterator (geometric sequence: a, a*r, a*r², ...) that stops when the value exceeds a given limit.
3. Add `DoubleEndedIterator` to `StepRange<i64>` so it can iterate from either end and support `.rev()`.
