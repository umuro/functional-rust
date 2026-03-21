📖 **[View on hightechmind.io →](https://hightechmind.io/rust/937-tail-recursive-accumulator)**

---

# 937-tail-recursive-accumulator — Tail-Recursive Accumulator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Naive recursion — `sum(list) = head + sum(tail)` — builds up a call stack one frame per element. For a list of 100,000 elements, this overflows the stack. The solution is tail recursion: make the recursive call the very last operation, enabling the compiler to reuse the current stack frame (tail-call optimization, TCO). OCaml guarantees TCO for tail-recursive functions. Rust does NOT guarantee TCO — the compiler may or may not optimize it. For stack safety in Rust, idiomatic code uses iterators (`.iter().sum()`) or explicit loops, which the compiler will never stack-overflow.

## Learning Outcomes

- Understand the difference between naive recursion and tail-recursive accumulator style
- Recognize that Rust does NOT guarantee tail-call optimization
- Use iterators as the idiomatic Rust alternative to TCO-dependent recursion
- Implement both OCaml-style accumulator and Rust-idiomatic iterator versions
- Compare OCaml's TCO guarantee with Rust's iterator-based stack safety

## Rust Application

`sum_naive` uses head + recursive call — not tail-recursive, can overflow. `sum_tr` uses an explicit loop with mutable slice and accumulator — equivalent to tail recursion but guaranteed stack-safe. `sum_iter` uses `list.iter().sum()` — the idiomatic approach. `rev_tr` uses `.iter().rev().cloned().collect()` — avoids recursion entirely. The lesson: in Rust, don't rely on TCO; use iterators or explicit loops for stack safety. The recursive versions are shown as OCaml-style comparisons, not production recommendations.

## OCaml Approach

OCaml guarantees TCO for functions where the recursive call is in tail position. `sum_tr acc = function | [] -> acc | x :: rest -> sum_tr (acc + x) rest` is tail-recursive and stack-safe for any list length. OCaml's `List.fold_left f init xs` is implemented this way. OCaml's `List.fold_right f xs init` is NOT tail-recursive (requires explicit `rev` + `fold_left`). The distinction between tail and non-tail recursive functions is more practically important in OCaml than in Rust.

## Key Differences

1. **TCO guarantee**: OCaml guarantees TCO for tail-recursive functions; Rust has no such guarantee — use iterators instead.
2. **Stack safety**: OCaml programs can be stack-safe using accumulator-style recursion; Rust programs should use iterators or explicit loops.
3. **Fold direction**: OCaml `List.fold_left` is tail-recursive (safe); `List.fold_right` is not (unsafe for large lists). Rust's `.fold()` is always O(1) stack.
4. **Practical difference**: In OCaml, writing tail-recursive functions is a required skill; in Rust, it's rarely necessary because iterator adapters cover all cases.

## Exercises

1. Implement `count_recursive(list: &[i32], pred: impl Fn(&i32) -> bool) -> usize` both naively and in accumulator style, and test with a 1M-element input.
2. Write a tail-recursive (loop-based) `flatten_recursive<T: Clone>(nested: &[Vec<T>]) -> Vec<T>` without using iterator `.flatten()`.
3. Implement `map_accumulate<T, U, A, F>(list: &[T], init: A, f: F) -> (Vec<U>, A)` that applies a stateful transform to each element using an explicit accumulator.
