📖 **[View on hightechmind.io →](https://hightechmind.io/rust/889-double-ended)**

---

# 889-double-ended — DoubleEndedIterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Most iterators traverse from front to back. Some algorithms benefit from bidirectional traversal: palindrome checking compares elements from both ends toward the center, taking the last N elements without knowing the total count, and simultaneous front-and-back consumption. Rust's `DoubleEndedIterator` extends `Iterator` with `next_back()`, enabling traversal from the end. This is implemented for slices, ranges, and many adapter types. The `.rev()` adapter wraps any `DoubleEndedIterator` to iterate in reverse. OCaml handles these cases with explicit `List.rev` or array indexing.

## Learning Outcomes

- Use `.rev()` to iterate in reverse over any DoubleEndedIterator
- Use `.next_back()` to consume from the end of a bidirectional iterator
- Consume from both ends simultaneously for palindrome-style algorithms
- Implement `take_last` and `ends` using back-iteration without index arithmetic
- Compare with OCaml's `List.rev` and array-based back-access

## Rust Application

`take_last` uses `.rev().take(n)` then `.rev()` to restore original order — getting the last n elements without knowing the total count. `last_element` uses `.next_back()` directly. `ends` creates one iterator and calls `iter.next()` for the first and `iter.next_back()` for the last — both consume from the same iterator simultaneously. `palindrome_check` alternates `iter.next()` and `iter.next_back()` in a loop, returning false if any pair mismatches, leveraging the fact that consumed ends narrow the iterator.

## OCaml Approach

OCaml lists are singly-linked and have no `DoubleEndedIterator` equivalent. Back-access requires `List.rev` (O(n) allocation) or converting to arrays first. Palindrome check: `list = List.rev list`. Taking last n elements: `List.filteri (fun i _ -> i >= len - n) list` (O(n) scan). Arrays support O(1) back-access via negative-equivalent indexing (`arr.(Array.length arr - 1 - i)`). OCaml's lack of bidirectional iteration for lists is a notable contrast.

## Key Differences

1. **Bidirectional without allocation**: Rust `next_back()` on a slice iterator is O(1) and zero-allocation; OCaml `List.rev` is O(n) and allocates.
2. **Simultaneous ends**: Rust allows consuming from both ends of the same iterator object; OCaml requires two separate list traversals.
3. **Adapter support**: Many Rust adapters (`.map()`, `.filter()`, `.chain()`) preserve `DoubleEndedIterator`; OCaml adapters always produce new lists.
4. **Slice specialization**: `SliceIter` implements `DoubleEndedIterator` directly; `.rev()` on an array-backed iterator is zero-cost.

## Exercises

1. Implement `longest_palindromic_prefix(s: &str) -> &str` that uses a double-ended char iterator to find the palindrome prefix.
2. Write `interleave_ends<T: Clone>(data: &[T]) -> Vec<T>` that alternates front and back elements: `[a0, an, a1, an-1, ...]`.
3. Implement `symmetric_filter<T: PartialEq + Clone>(data: &[T], pred: impl Fn(&T) -> bool) -> Vec<T>` that uses a double-ended iterator to filter from both ends simultaneously.
