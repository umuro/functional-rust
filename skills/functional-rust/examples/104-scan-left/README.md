# 104: Scan Left — Running Accumulation

**Difficulty:** 1  **Level:** Foundations

Produce all intermediate results of a fold — not just the final answer.

## The Problem This Solves

`fold` gives you the end result. But often you need the journey, not just the destination: a running bank balance after each transaction, the maximum temperature seen so far at each data point, a cumulative sum for a progress bar.

`scan` is "fold with history." Where `fold([1,2,3,4,5], 0, +)` gives `15`, `scan([1,2,3,4,5], 0, +)` gives `[0, 1, 3, 6, 10, 15]` — the accumulator at each step, including the initial value.

The relationship: `fold` returns the *last element* of what `scan` returns.

## The Intuition

Picture a running total on a cash register receipt: each line shows the subtotal after adding that item. The final total is the last line. Scan gives you every line; fold gives you only the last.

Rust's `Iterator::scan` does this lazily — it yields intermediate accumulator values one at a time, only computing what's consumed. This means you can scan an infinite iterator and `take` as many results as you need.

## How It Works in Rust

```rust
// Running sum: [1,2,3,4,5] → [0, 1, 3, 6, 10, 15]
pub fn running_sum(xs: &[i32]) -> Vec<i32> {
    let mut acc = 0;
    std::iter::once(0)   // include the initial value
        .chain(xs.iter().map(move |&x| {
            acc += x;
            acc
        }))
        .collect()
}

// Running maximum using Iterator::scan
pub fn running_max(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .scan(i32::MIN, |state, &x| {
            *state = (*state).max(x);  // update the state
            Some(*state)               // yield the new state
        })
        .collect()
}
```

Generic `scan_left` that mirrors OCaml directly:

```rust
pub fn scan_left<T: Clone, U: Clone>(
    f: impl Fn(&T, &U) -> T,
    init: T,
    xs: &[U],
) -> Vec<T> {
    let mut result = vec![init.clone()];  // start with initial value
    let mut acc = init;
    for x in xs {
        acc = f(&acc, x);
        result.push(acc.clone());
    }
    result
}
```

Usage:
```rust
scan_left(|a, b| a + b, 0, &[1, 2, 3, 4, 5])
// → [0, 1, 3, 6, 10, 15]
```

## What This Unlocks

- **Financial ledgers** — balance after each transaction
- **Watermark tracking** — running max/min over a data stream
- **Prefix sums** — O(1) range sum queries after O(n) preprocessing

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in scan | No — build with `fold_left` | `Iterator::scan` built in |
| Laziness | Eager (builds list) | Lazy — yields one value at a time |
| Initial value | Included in output | Up to implementation |
| State mutation | Immutable accumulator | `scan` takes `&mut State` |
| Relation to fold | Last element of scan result | Same |
