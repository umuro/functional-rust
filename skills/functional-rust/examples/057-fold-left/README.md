# 057: Fold Left

**Difficulty:** 2  **Level:** Beginner

Accumulate a result left-to-right with a running accumulator — the tail-recursive fold.

## The Problem This Solves

Sum a list of numbers. Find the maximum. Reverse a list. Build a string from parts. All of these are left folds: start with an initial value, walk the list left to right, update the accumulator at each step, return the final accumulated result.

`fold_left` is the workhorse of functional programming. It's the pattern behind `sum`, `product`, `max`, and `reverse`. Once you see `fold_left`, you see it everywhere. And unlike `fold_right`, it's tail-recursive — safe for arbitrarily large inputs because it doesn't build up a stack.

Rust's `Iterator::fold` *is* `fold_left`. Every time you call `.fold(init, |acc, x| ...)`, you're writing a left fold.

## The Intuition

`fold_left f init [a, b, c]` evaluates as:

```
((init f a) f b) f c
```

Left to right. The accumulator starts at `init`, is updated by `f(acc, a)`, then `f(result, b)`, then `f(result, c)`. Each step consumes the current accumulator and produces the next one.

For sum: `((0 + 3) + 1) + 4 = 8`. The accumulator carries the running total. For reverse: start with `[]`, prepend each element: `[] → [a] → [b,a] → [c,b,a]`.

## How It Works in Rust

```rust
// Generic fold_left — iterative (Rust has no TCO guarantee)
pub fn fold_left<T, A>(f: impl Fn(A, &T) -> A, mut acc: A, xs: &[T]) -> A {
    for x in xs {
        acc = f(acc, x);  // tail-recursive in OCaml; iterative loop in Rust
    }
    acc
}

// Sum, product, max via fold_left:
fold_left(|acc, &x| acc + x, 0, &[1, 2, 3])    // → 6
fold_left(|acc, &x| acc * x, 1, &[1, 2, 3])    // → 6
// Max with Option for empty-safety:
let (&first, rest) = xs.split_first()?;
fold_left(|acc, &x| acc.max(x), first, rest)   // → Some(max)

// Reverse via fold_left — the classic FP trick:
fold_left(|mut acc: Vec<i64>, &x| { acc.insert(0, x); acc }, vec![], xs)

// Rust's built-in left fold:
xs.iter().copied().fold(0i64, i64::wrapping_add)
```

Note: OCaml guarantees tail-call optimisation (TCO) for `fold_left`. Rust does not. Our implementation uses a `for` loop, which has the same constant-stack behaviour without relying on TCO.

## What This Unlocks

- **Universal aggregation** — any "combine a list into a single value" operation is a fold: sum, product, max, count, string join.
- **Building data structures** — use fold to construct maps, sets, histograms from a sequence.
- **Understanding `Iterator::fold`** — Rust's standard fold is right there in the iterator chain; this example makes its mechanics explicit.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tail recursion | Guaranteed TCO — safe for millions of elements | No TCO — use iterative loop instead |
| Empty-list max | `List.hd` panics on empty | Return `Option<T>` — explicit emptiness |
| Accumulator | Immutable, returned each step | `mut acc` updated in place each iteration |
| In-place reverse | Allocates new list | `Vec::reverse()` — O(1) extra space |
| Standard library | `List.fold_left f init lst` | `iter().fold(init, \|acc, x\| ...)` |
