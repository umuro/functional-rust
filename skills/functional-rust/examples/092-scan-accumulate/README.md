# 092: Scan / Accumulate

**Difficulty:** 3  **Level:** Intermediate

Like `fold` but emits every intermediate value — running sums, cumulative statistics, balance histories.

## The Problem This Solves

`fold` (or `reduce`) collapses a sequence to a single value. But often you want the whole trajectory: every partial sum, every running maximum, every balance after each transaction. Looping with a mutable accumulator and pushing to a separate vector works but separates the "what" from the "how."

`scan` keeps state across elements and yields each intermediate state as part of the output iterator. It's the right tool for: financial ledgers, moving statistics, encoding state machines, or any computation where the history matters as much as the result.

## The Intuition

Think of `scan` as `fold` with a memory dump after each step. If `fold` is a reduce that returns the final answer, `scan` returns the answer *at every step along the way*.

Python's `itertools.accumulate(data, func)` is the direct equivalent. JavaScript has no standard equivalent — you'd write `.reduce` and push to an array manually.

## How It Works in Rust

```rust
// Running sum: each element is the sum so far
fn running_sum(data: &[i32]) -> Vec<i32> {
    data.iter().scan(0, |acc, &x| {
        *acc += x;    // mutate the state in place
        Some(*acc)    // yield current state; return None to stop early
    }).collect()
}

// Running max: track the highest value seen so far
fn running_max(data: &[i32]) -> Vec<i32> {
    data.iter().scan(i32::MIN, |max, &x| {
        *max = (*max).max(x);
        Some(*max)
    }).collect()
}

// Balance history: start at 0, apply each transaction
fn balance_history(transactions: &[i32]) -> Vec<i32> {
    // Prepend starting balance using chain
    std::iter::once(0).chain(
        transactions.iter().scan(0, |bal, &tx| {
            *bal += tx;
            Some(*bal)
        })
    ).collect()
}
```

The closure receives `|state: &mut S, item: &Item|`. Mutate `state` in place and return `Some(value)` to yield, `None` to stop early. The `state` outlives each element — it carries over.

## What This Unlocks

- **Financial ledgers**: running balance after each deposit/withdrawal.
- **Statistics**: cumulative mean, variance, or max without storing all data.
- **State machines**: emit the current state at each transition — useful for debugging pipelines.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in scan | No (manual fold) | `.scan(init, \|state, item\| ...)` |
| State mutation | Accumulated list | `&mut S` in closure |
| Early termination | Pattern match on list | Return `None` from closure |
| With index | Manual counter | `.enumerate()` then `.scan()` |
| Python equivalent | `itertools.accumulate` | `.scan()` |
