📖 **[View on hightechmind.io →](https://hightechmind.io/rust/005-reverse-list)**

---

# 005 — Reverse a List

## Problem Statement

Reversing a list is a deceptively simple problem that illuminates a critical performance trap in functional programming: naive recursive reverse is O(n²) because each recursive call appends to the end. The solution — the accumulator pattern — is O(n) and is the prototype for tail-recursive programming. Understanding this transformation is essential before working with any recursive data structure.

The accumulator pattern generalizes: it replaces "build result on the way back up the stack" with "carry the result forward as a parameter". This is exactly what makes a function tail-recursive, allowing the compiler to optimize it into a loop. This pattern appears in reverse, flatten, map, filter, and virtually every recursive list operation.

## Learning Outcomes

- Implement list reversal in four ways: built-in, iterator, fold, and recursive
- Understand why naive recursive reverse is O(n²) and how the accumulator fixes it
- Use the `fold` pattern to carry state forward through a list
- Recognize when recursion can be replaced with an accumulator for stack safety
- Understand Rust's in-place `reverse()` vs immutable iterator-based reversal

## Rust Application

`reverse_inplace` uses `Vec::reverse()` — O(n), in-place, no allocation. `reverse_iter` uses `.iter().rev().copied().collect()` — O(n), allocates a new `Vec`. `reverse_fold` uses `fold` with `acc.insert(0, x)` — this is O(n²) because `insert(0, _)` shifts all elements; it demonstrates the concept but is not efficient. `reverse_recursive` is the naive O(n²) version: it builds the reversed tail first, then pushes the head. The key insight is that the accumulator-based `rev_acc` eliminates this by prepending to the accumulator instead.

## OCaml Approach

OCaml's standard library provides `List.rev` and `List.rev_append`. The classic teaching version is `let rec rev_acc acc = function [] -> acc | x :: t -> rev_acc (x :: acc) t`. Because OCaml guarantees tail-call optimization, `rev_acc [] lst` is compiled into a loop. The `List.fold_left` version `List.fold_left (fun acc x -> x :: acc) [] lst` is equivalent and idiomatic.

## Key Differences

1. **In-place vs functional**: Rust's `Vec::reverse()` mutates in place — O(n), zero allocation. OCaml's `List.rev` always allocates a new list (immutable data structure constraint).
2. **Tail-call optimization**: OCaml guarantees TCO for tail-recursive functions. Rust does not — the accumulator-based version should be implemented as a loop, not as recursion, for large inputs.
3. **Stack risk**: A naive recursive reverse on a list of 100,000 elements will stack overflow in Rust but not in OCaml (after TCO).
4. **`fold_left` direction**: Both languages have left-fold and right-fold. Using `fold_left` with cons (`x :: acc`) naturally builds a reversed list — this is a fundamental pattern.

## Exercises

1. **Tail-safe recursive reverse**: Rewrite `reverse_recursive` using an explicit accumulator argument so it is tail-recursive in structure (even though Rust won't TCO it, understand the pattern).
2. **Palindrome check**: Use `reverse_iter` to implement `is_palindrome(v: &[i32]) -> bool` in one line. Then implement a zero-copy version using `v.iter().eq(v.iter().rev())`.
3. **Rotate**: Write `rotate_left(v: &[i32], n: usize) -> Vec<i32>` that moves the first `n` elements to the end, using slices and `extend` rather than repeated reversal.
