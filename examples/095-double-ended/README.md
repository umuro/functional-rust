[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 095 — Double-Ended Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Use Rust's `DoubleEndedIterator` trait to iterate from both ends simultaneously. Implement palindrome detection by comparing forward and reversed iterators, and demonstrate `next()` / `next_back()` on the same iterator instance. Compare with OCaml's manual array-based bidirectional iteration.

## Learning Outcomes

- Use `v.iter().rev()` to create a reversed iterator from a `DoubleEndedIterator`
- Compare two iterators with `.eq(other_iter)` for element-wise equality
- Use `.next_back()` to advance from the back without consuming the front
- Understand that `next()` and `next_back()` share position state — they meet in the middle
- Map Rust's `DoubleEndedIterator` to OCaml's mutable front/back array cursors
- Recognise when double-ended iteration avoids a reversal allocation

## Rust Application

`v.iter().eq(v.iter().rev())` checks palindrome: the forward and reversed iterators are paired element-by-element. `take_from_both` demonstrates calling both `next()` and `next_back()` on the same mutable `iter` — the cursor meets in the middle. After taking 2 from the front and 2 from the back of `[1,2,3,4,5]`, the middle element `3` remains. `v.iter().rev()` is O(1) — it just flips the direction, no allocation.

## OCaml Approach

OCaml lists are singly linked — no efficient reverse iteration. The `iter_both` function simulates double-ended iteration on arrays using mutable `front` and `back` index references. `is_palindrome_arr` checks symmetry with array indexing `arr.(i) = arr.(n - 1 - i)`. Rust's built-in protocol is cleaner; OCaml requires manual bookkeeping.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Reverse iteration | `.rev()` on any `DoubleEndedIterator` | Arrays: `arr.(n-1-i)`, lists: reverse |
| Bidirectional | `next()` + `next_back()` on same iter | Manual front/back indices |
| Palindrome | `.iter().eq(.iter().rev())` | `arr.(i) = arr.(n-1-i)` |
| Allocation | Zero (`.rev()` is O(1)) | Zero for arrays |
| Protocol | Trait method `next_back` | Manual cursor |
| Lists | Not `DoubleEndedIterator` | `List.rev` makes a copy |

`DoubleEndedIterator` is implemented by slices, ranges, `Vec`, `VecDeque`, `Rev`, and many standard adapters. It enables efficient palindrome checks, bidirectional parsing, and simultaneous front/back consumption without allocating a reversed copy.

## Exercises

1. Implement `is_palindrome_str(s: &str) -> bool` using `s.chars().eq(s.chars().rev())`.
2. Write `zip_ends<T: Clone>(v: &[T]) -> Vec<(T, T)>` that pairs the first element with the last, second with second-to-last, etc.
3. Implement a bidirectional deque processor that alternately takes from front and back.
4. Use `DoubleEndedIterator` to implement a `rotate_left` that moves the first `n` elements to the back.
5. In OCaml, implement bidirectional iteration over a doubly linked list, defining both `next` and `prev` operations.
