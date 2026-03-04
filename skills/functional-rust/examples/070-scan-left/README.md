# 070: Scan Left

**Difficulty:** ⭐  **Level:** Foundations

Like `fold`, but keep every intermediate value — running totals, prefix sums, and accumulated state made simple.

## The Problem This Solves

You have a list of daily sales figures and you want the running total after each day, not just the grand total. Or you want the running maximum as you scan through sensor readings. Or you want to watch how an accumulator evolves step by step.

`fold` collapses a list to a single value. `scan` does the same accumulation, but captures every intermediate state. The result is a list as long as the input (plus one for the initial value).

In Python, `itertools.accumulate` does this. In JavaScript you'd build the array manually. In Rust, `Iterator::scan` is built-in — though its API is slightly different from the functional ideal, so it's instructive to write your own `scan_left` first.

## The Intuition

Run a fold step-by-step. After each step, save the current accumulator value. At the end you have all the intermediate results, not just the final one.

Input `[1, 2, 3, 4, 5]` with addition starting from `0`:
- Start: `[0]`
- After 1: `[0, 1]`
- After 2: `[0, 1, 3]`
- After 3: `[0, 1, 3, 6]`
- After 4: `[0, 1, 3, 6, 10]`
- After 5: `[0, 1, 3, 6, 10, 15]`

That's your prefix sum. The same pattern works for running max, running product, string accumulation, anything.

## How It Works in Rust

Custom `scan_left` — the clearest version:

```rust
pub fn scan_left<T, A, F>(init: A, items: &[T], f: F) -> Vec<A>
where
    A: Clone,
    F: Fn(&A, &T) -> A,
{
    let mut result = vec![init.clone()];  // include the initial value
    let mut acc = init;
    for item in items {
        acc = f(&acc, item);
        result.push(acc.clone());
    }
    result
}

// Running sum: scan_left with addition
pub fn running_sum(nums: &[i64]) -> Vec<i64> {
    scan_left(0i64, nums, |acc, x| acc + x)
}
// running_sum(&[1,2,3,4,5]) → [0, 1, 3, 6, 10, 15]
```

Using the built-in `Iterator::scan` adapter:

```rust
// scan takes FnMut with a mutable state reference
let running = nums.iter().scan(0i64, |state, &x| {
    *state += x;
    Some(*state)
});
// Yields: 1, 3, 6, 10, 15  (no initial value included)
```

Note: the built-in `scan` doesn't include the initial value; add it manually if needed.

## What This Unlocks

- **Running totals** — balance after each transaction, score after each round
- **Prefix sums** — precompute cumulative sums for O(1) range sum queries
- **State traces** — log every state of a simulation or state machine for debugging or visualization

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Implementation | `fold_left` accumulating both value and result list | Custom `scan_left` or built-in `Iterator::scan` |
| Includes initial | `[init] @ accumulated_list` | `vec![init]` then extend |
| Built-in | No `scan_left` in stdlib | `Iterator::scan` (FnMut, no initial in output) |
| Functional purity | Pure — no mutation | Custom version is pure; built-in uses `FnMut` |
