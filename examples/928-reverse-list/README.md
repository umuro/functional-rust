📖 **[View on hightechmind.io →](https://hightechmind.io/rust/928-reverse-list)**

---

# 928-reverse-list — Reverse List
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Reversing a list is the "hello world" of functional data structure algorithms. It exercises the core skill of list recursion and accumulator-based tail recursion. The naive recursive version (`rev(t) ++ [h]`) is O(n²) due to append at each step. The accumulator version (`rev_acc(t, h::acc)`) is O(n) and tail-recursive — the standard functional programming solution. OCaml's `List.rev` uses this pattern. Rust's `.rev()` iterator adapter is O(1) (lazy reversal); `.iter().rev().collect()` is O(n). This example shows all three approaches.

## Learning Outcomes

- Implement list reversal using Rust's iterator `.rev()` adapter
- Implement tail-recursive reversal with an accumulator using slice patterns
- Understand why the naive recursive reversal is O(n²) while the accumulator version is O(n)
- Use `list.reverse()` for in-place mutation as the imperative style
- Compare with OCaml's `List.rev` and the functional accumulator pattern

## Rust Application

`rev` uses `list.iter().rev().cloned().collect()` — idiomatic Rust. `rev_mut` uses `list.reverse()` for in-place reversal (O(n), zero allocation). `rev_fold` uses `.fold()` to prepend to a `Vec` using `insert(0, ...)` — this is O(n²) and shown as an anti-pattern. `rev_recursive` uses `aux` with a `Vec` accumulator, matching OCaml's `rev_append` pattern. The recursive version uses `[h, rest @ ..]` slice patterns to destructure head and tail.

## OCaml Approach

`List.rev: 'a list -> 'a list` is the standard library function, implemented as: `let rev xs = let rec aux acc = function | [] -> acc | x :: rest -> aux (x :: acc) rest in aux [] xs`. The accumulator builds the reversed list by prepending, which is O(1) per element. `List.rev_append xs acc` reverses `xs` onto `acc` in one pass. OCaml's tail-call optimization makes the accumulator version stack-safe for any length.

## Key Differences

1. **In-place vs functional**: Rust has `reverse()` for in-place mutation (not available for OCaml's immutable lists); OCaml always creates a new list.
2. **Iterator laziness**: Rust `.rev()` is lazy — it reverses the traversal direction without allocation until `.collect()` is called; OCaml's `List.rev` is always eager.
3. **Accumulator style**: Both languages use the accumulator pattern for O(n) functional reversal; Rust uses an explicit `Vec` as the accumulator.
4. **Stack safety**: OCaml relies on TCO for the accumulator version; Rust's iterator approach avoids deep recursion entirely.

## Exercises

1. Implement `rev_words(sentence: &str) -> String` that reverses the word order but not individual characters.
2. Write `rotate_left<T: Clone>(data: &[T], n: usize) -> Vec<T>` using rev operations on two sub-slices.
3. Implement `palindrome_indices(data: &[i32]) -> Vec<usize>` using a reverse to find all elements that appear at the same position in the reversed array.
