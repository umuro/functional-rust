# 067: Lazy Sequences

**Difficulty:** ⭐⭐  **Level:** Foundations

Work with infinite sequences efficiently — because Rust iterators compute nothing until you ask.

## The Problem This Solves

You want the first 10 prime numbers. Or the Fibonacci numbers up to 1,000. Or every number in a Collatz sequence. The sequence is potentially infinite, but you only need a finite slice of it.

The eager approach allocates a huge vector, fills it, then takes what you need. That's wasteful (maybe impossible for truly infinite sequences) and couples your "generate" logic to your "consume" logic.

The lazy approach defines *how* to generate the sequence, without generating anything. You attach filters, maps, and transformations. Only when you say "give me 10 elements" does any computation happen — and only enough to produce those 10.

## The Intuition

Rust iterators are lazy by default. When you write `(0..).filter(is_prime).take(10)`, nothing is computed yet. It's a description of work, not the work itself. Only `.collect()` (or `.for_each()`, `.sum()`, etc.) actually drives the computation.

This means you can chain 10 operations on a million-element sequence and only traverse the data once. Each element flows through the entire pipeline before the next one starts.

Python has the same idea with generators (`yield`). JavaScript has it with generator functions (`function*`). OCaml needed the `Seq` module (4.14+) to get comparable ergonomics. Rust has it built into every iterator from day one.

## How It Works in Rust

```rust
// Infinite naturals — no allocation, computes on demand
pub fn naturals(start: u64) -> impl Iterator<Item = u64> {
    start..  // range with no upper bound
}

// Infinite Fibonacci using successors — each step derives from the previous
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}

// Consume lazily — computes only what's needed
let first_10_fibs: Vec<u64> = fibs().take(10).collect();
let sum_of_first_100: u64 = naturals(1).take(100).sum();
```

The `from_fn` approach for custom sequences:

```rust
// Custom lazy generator with mutable local state
let mut state = (0u64, 1u64);
let fibs = std::iter::from_fn(move || {
    let next = state.0;
    state = (state.1, state.0 + state.1);
    Some(next)
});
```

## What This Unlocks

- **Infinite sequences** — primes, Fibonacci, Collatz — define them once, take what you need
- **Efficient pipelines** — chain `filter`, `map`, `flat_map` with a single traversal, no intermediate allocations
- **Event streams** — model a stream of incoming data as a lazy iterator; process items without buffering everything

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy sequence type | `Seq.t` (OCaml 4.14+) | Every `Iterator` is lazy |
| Infinite sequence | `Seq.unfold` | `std::iter::successors` / `from_fn` |
| Build then consume | `Seq.take n \|> Seq.to_list` | `.take(n).collect()` |
| Triggering computation | Consuming the `Seq.t` | `.collect()` / `.for_each()` / `.sum()` etc. |
