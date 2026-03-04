# 068: Tail-Recursive Accumulator

**Difficulty:** 2  **Level:** Intermediate

Transform naive recursion into tail recursion by passing an accumulator — and understand why Rust prefers iterators.

## The Problem This Solves

Sum a list of a million numbers recursively and you'll get a stack overflow. Each recursive call pushes a stack frame, and the call stack has a hard limit (typically 8MB). Naive `sum([1, 2, ..., 1_000_000])` tries to push a million frames.

The classic fix is the **accumulator pattern**: instead of returning a value and adding to it on the way *back up* the call stack, carry a running total as a parameter on the way *down*. The final recursive call can then return the accumulator directly — no work left to do on the return path. This is a **tail call**, and a compiler that implements tail-call optimization (TCO) can reuse the same stack frame, making the recursion as safe as a loop.

OCaml guarantees TCO for tail-recursive functions. Rust does not. So in Rust, the accumulator pattern teaches the concept, but for production code you should use iterators (`list.iter().sum()`) or explicit loops. This example shows both, explains why, and builds the transferable skill of recognizing when a function is tail-recursive.

## The Intuition

**Non-tail-recursive sum**: `sum([1, 2, 3]) = 1 + sum([2, 3]) = 1 + (2 + sum([3])) = 1 + (2 + (3 + sum([])))`. The additions happen *after* returning from the recursive calls. Each level waits for the level below before it can compute. Stack depth = list length.

**Tail-recursive sum with accumulator**: `sum_tr(acc=0, [1, 2, 3]) = sum_tr(acc=1, [2, 3]) = sum_tr(acc=3, [3]) = sum_tr(acc=6, []) = 6`. The recursive call is the *last thing* that happens — no work left after returning. With TCO, this becomes a `goto` to the top of the function, reusing the same stack frame.

The pattern to recognize: a function is tail-recursive if every recursive call is in **tail position** — the result of the call is returned directly, not used in a further computation.

In Rust without TCO guarantee, the accumulator pattern is still instructive because: (1) it shows you how to think about computation in terms of state that flows forward rather than results that accumulate backward, and (2) it's directly translatable to an explicit loop.

## How It Works in Rust

Naive (not tail-recursive — dangerous for large lists):
```rust
pub fn sum_naive(list: &[i64]) -> i64 {
    match list {
        [] => 0,
        [h, rest @ ..] => h + sum_naive(rest),  // + happens AFTER return — not tail position
    }
}
```

Tail-recursive style (accumulator carries state forward):
```rust
pub fn sum_tr(list: &[i64]) -> i64 {
    fn go(acc: i64, slice: &[i64]) -> i64 {
        match slice {
            [] => acc,                         // return accumulator directly
            [h, rest @ ..] => go(acc + h, rest), // last operation: recursive call only
        }
    }
    go(0, list)  // start with acc=0
}
```

The idiomatic Rust version (what you'd actually write):
```rust
pub fn sum_iter(list: &[i64]) -> i64 { list.iter().sum() }
```

Reverse as accumulator pattern (OCaml's `h :: acc` prepend → Rust's push + reverse):
```rust
pub fn rev_tr<T: Clone>(list: &[T]) -> Vec<T> {
    // Idiomatic: iterators handle this cleanly
    list.iter().rev().cloned().collect()
}
```

When you really need the recursive accumulator style (e.g., teaching or implementing without std), convert to a loop:
```rust
// The tail-recursive pattern translates directly to a loop:
pub fn sum_loop(list: &[i64]) -> i64 {
    let mut acc = 0;
    let mut rest = list;
    loop {
        match rest {
            [] => return acc,
            [h, tail @ ..] => { acc += h; rest = tail; }
        }
    }
}
```

## What This Unlocks

- **Stack-safe computation**: understanding tail position lets you convert any recursive algorithm to a safe iterative one — essential for processing deeply nested structures.
- **Mental model for loops**: tail-recursive functions and `while` loops are the same computation — one is declarative, one is imperative.
- **Recognizing when OCaml code needs `_tr` variants**: OCaml standard library has `List.fold_left` (tail-recursive) and effectively discourages `List.fold_right` for large lists — now you know why.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| TCO guarantee | Yes — tail calls are optimized | No — LLVM *may* optimize, not guaranteed |
| Accumulator pattern | Same structure; safe due to TCO | Instructive but use iterators for safety |
| `List.rev` | Uses `rev_append` (tail-recursive) | `iter().rev().cloned().collect()` |
| Preferred idiom | Tail-recursive helpers with `go` | `iter().sum()`, `iter().fold()`, explicit loops |
| Stack overflow risk | Eliminated by TCO for tail calls | Still present; default stack ~8MB |
