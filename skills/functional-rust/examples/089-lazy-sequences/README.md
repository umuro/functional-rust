# 089: Lazy Sequences

**Difficulty:** 2  **Level:** Intermediate

Build infinite sequences — natural numbers, primes, triangle numbers, powers — and consume exactly as many elements as you need with `take`, `take_while`, and `find`.

## The Problem This Solves

Some sequences are naturally infinite: the natural numbers, prime numbers, Fibonacci, powers of two. To work with them you need laziness — a way to describe "all primes" without computing them all upfront (which would take forever).

The imperative approach is a stateful loop with a break condition. You need to track state manually, mix generation with consumption, and repeat the same boilerplate for every sequence. The result is code where the algorithm for *generating* the sequence is tangled with the logic for *using* it.

Lazy iterators separate these concerns cleanly. Define how to produce the next element; let the consumer decide how many to take. The generator runs only as far as needed.

## The Intuition

In Python, `itertools.count()` is an infinite iterator; generator functions with `yield` create lazy sequences. In OCaml, `Seq.t` is a lazy type — a `unit -> node` function that produces elements on demand. In Haskell, all lists are lazy by default.

Rust's iterators are lazy by design — nothing runs until consumed. This means `(0u64..).filter(is_prime)` really is "all primes" as an iterator. It doesn't compute anything until you call `.take(10)` or `.find(|p| *p > 100)`.

The difference from Haskell: Rust iterators are *pull-based* (the consumer drives) and *stateful* (mutable `next()` method). The laziness is explicit — you know exactly when computation happens.

## How It Works in Rust

```rust
// Infinite ranges — the simplest lazy sequence
fn naturals() -> impl Iterator<Item = u64> {
    0u64..   // range with no upper bound — lazy, computes nothing yet
}

// Compose lazy sequences
fn squares() -> impl Iterator<Item = u64> {
    naturals().map(|n| n * n)   // still lazy — just describes the transformation
}
```

```rust
// Filter over an infinite sequence — still lazy
fn primes() -> impl Iterator<Item = u64> {
    naturals().filter(|&n| is_prime(n))
}

// Nothing computed until we consume with take()
let first_ten: Vec<u64> = primes().take(10).collect();
let primes_below_100: Vec<u64> = primes().take_while(|&p| p < 100).collect();
let first_big_prime = primes().find(|&p| p > 1_000_000);
```

```rust
// successors: generate sequences where each element depends on the previous
fn powers_of(base: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(1u64), move |&prev| {
        prev.checked_mul(base)  // returns None on overflow — sequence ends naturally
    })
}

// scan: like fold but yields each intermediate accumulator
fn triangle_numbers() -> impl Iterator<Item = u64> {
    (1u64..).scan(0u64, |acc, n| {
        *acc += n;
        Some(*acc)   // yields running sum: 1, 3, 6, 10, 15, ...
    })
}
```

```rust
// Compose multiple lazy operations — still nothing computed
let result: Vec<u64> = naturals()
    .filter(|n| n % 3 == 0)    // multiples of 3
    .map(|n| n * n)              // their squares
    .take_while(|&n| n < 1000)  // only those below 1000
    .collect();                  // NOW it computes — just enough elements
```

## What This Unlocks

- **On-demand generation**: produce only as many elements as the caller needs — no waste, no upfront cost for sequences that might be cut short.
- **Composable sequence definitions**: `primes().filter(|p| p % 4 == 3)` — Gaussian primes, defined in one line by composing simpler lazy sequences.
- **Search without bounds**: `primes().find(|&p| p > threshold)` — finds the answer without knowing upfront how far to search.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy sequence type | `Seq.t = unit -> node` | `impl Iterator<Item = T>` |
| Infinite sequence | `Seq.unfold` | `(0..)` range or `iter::successors` |
| Take N elements | `Seq.take` | `.take(n)` |
| Take while condition | `Seq.take_while` | `.take_while(pred)` |
| Running accumulator | `Seq.scan` or manual | `.scan(init, f)` |
| When computation happens | On each `Seq.next` call | On each `Iterator::next()` call |
