# 103: Unfold — Generating Sequences from Seeds

**Difficulty:** 2  **Level:** Intermediate

Build sequences from a starting value and a step function — the categorical dual of `fold`.

## The Problem This Solves

`fold` consumes a collection to produce one value. `unfold` does the opposite: it produces a collection from one value. Given a seed and a function that says "what's the next value and what's the next seed?", you get a sequence.

This pattern shows up constantly: the Collatz sequence, pagination (each page is a "seed" for the next), game replays (each state produces the next), tree traversal order. Anywhere you have a "current state → (output, next state)" shape, unfold is the right abstraction.

## The Intuition

Think of unfold as running a state machine: you start with a seed, the function produces one output item and a new seed, then it runs again with the new seed, until the function returns `None` to stop.

```
seed → f(seed) = Some((item1, seed2))
seed2 → f(seed2) = Some((item2, seed3))
seed3 → f(seed3) = None  → stop
Result: [item1, item2]
```

The lazy version (returning an `Iterator`) only computes values when consumed — useful for potentially infinite sequences.

## How It Works in Rust

Eager version — collects into a `Vec`:

```rust
pub fn unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T> {
    let mut result = Vec::new();
    let mut state = seed;
    loop {
        match f(state) {
            None => break,
            Some((value, next)) => {
                result.push(value);
                state = next;
            }
        }
    }
    result
}

// Generate a range [a..=b]
pub fn range(a: i32, b: i32) -> Vec<i32> {
    unfold(a, |i| if i > b { None } else { Some((i, i + 1)) })
}

// Collatz sequence
pub fn collatz(n: u64) -> Vec<u64> {
    unfold(n, |x| {
        if x == 0 { None }
        else if x == 1 { Some((1, 0)) }                              // stop after 1
        else { Some((x, if x % 2 == 0 { x / 2 } else { 3 * x + 1 })) }
    })
}
```

Lazy version using `from_fn` — returns an iterator, computes on demand:

```rust
pub fn unfold_iter<S, T>(
    seed: S,
    f: impl Fn(&S) -> Option<(T, S)>,
) -> impl Iterator<Item = T> {
    let mut state = Some(seed);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}
```

`std::iter::successors` is the standard library's built-in unfold for sequences where the value *is* the state:

```rust
let collatz = std::iter::successors(Some(6u64), |&x| {
    if x <= 1 { None }
    else if x % 2 == 0 { Some(x / 2) }
    else { Some(3 * x + 1) }
});
```

## What This Unlocks

- **Pagination** — `unfold(page_1_url, |url| fetch_page(url).map(|p| (p.items, p.next_url)))`
- **State machine replay** — generate a sequence of states from an initial state and transition function
- **Mathematical sequences** — any recurrence relation (Fibonacci, Collatz, continued fractions)

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Eager unfold | Recursive `let rec` building a list | Loop with `Vec::push` |
| Lazy unfold | `Seq.unfold` (since 4.07) | `std::iter::from_fn` |
| Built-in variant | `Seq.unfold` | `std::iter::successors` |
| Stop condition | `None` return | `None` return |
| Memory | GC manages intermediate state | `Option<S>` moves state — no GC needed |
