# 101: Lazy Sequences

**Difficulty:** 2  **Level:** Intermediate

Work with infinite sequences — Fibonacci numbers, primes, naturals — without computing all of them upfront.

## The Problem This Solves

Some problems are naturally expressed as infinite sequences: all prime numbers, all Fibonacci numbers, an infinite stream of events. You only ever need a finite prefix, but you want to describe the sequence as if it were infinite and let the consumer decide how much to take.

Eager evaluation fails here — you can't put infinitely many Fibonacci numbers in a `Vec`. You need a structure that computes values *on demand*, only when consumed.

This is the distinction between "compute now" and "compute when needed" — the latter is called lazy evaluation.

## The Intuition

Rust's iterators are lazy by default. Writing `(0..)` doesn't allocate an infinite list — it creates a description of *how to produce* natural numbers. The numbers are only computed when you call `.next()` on the iterator, which happens inside `.take(10).collect()`.

This is different from OCaml, where you need the explicit `Seq` module to opt into laziness. In Rust, *every* iterator is lazy. The pipeline `naturals().filter(is_prime).take(10).collect()` runs the filter and take in a single pass — no intermediate lists.

## How It Works in Rust

```rust
// Infinite natural numbers — just a range
pub fn naturals(start: u64) -> impl Iterator<Item = u64> {
    start..   // lazy — no values computed yet
}

// Fibonacci via successors — each step produces (current, next_pair)
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| {
        a.checked_add(b).map(|s| (b, s))  // stop if overflow
    })
    .map(|(a, _)| a)  // extract just the current value
}

// Primes via filter — lazy sieve (simple, not the most efficient)
pub fn primes() -> impl Iterator<Item = u64> {
    (2..).filter(|&n| is_prime(n))  // only compute is_prime when needed
}
```

Using them:

```rust
let first_10_fibs: Vec<u64> = fibs().take(10).collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

// Primes between 100 and 200
let primes_100_200: Vec<u64> = primes()
    .skip_while(|&p| p < 100)
    .take_while(|&p| p <= 200)
    .collect();
```

You can also write generic unfold using `from_fn`:

```rust
pub fn unfold<S, T>(seed: S, f: impl Fn(&S) -> Option<(T, S)>) -> impl Iterator<Item = T> {
    let mut state = Some(seed);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}
```

## What This Unlocks

- **Stream processing** — process event streams, paginated APIs, or sensor data without loading everything into memory
- **Combinatorial generation** — enumerate permutations, combinations, or search trees lazily
- **Mathematical sequences** — express any recurrence relation as an iterator

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy by default | No — use `Seq` module explicitly | Yes — all iterators are lazy |
| Infinite range | `Seq.ints 0` (since 4.07) | `0..` built-in range |
| Unfold | `Seq.unfold` | `std::iter::from_fn` or `successors` |
| Consuming | `Seq.take n seq |> List.of_seq` | `.take(n).collect::<Vec<_>>()` |
| Overhead | Closure allocation per step | Zero-cost abstractions (monomorphized) |
