# Example 1000: Option Type — Safe List Maximum
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Find the maximum element in a list without raising an exception on empty input. Return `None` for empty lists and `Some(max_value)` otherwise. This demonstrates the Option type as a safe alternative to exceptions or sentinel values.

## Learning Outcomes

- How `Option<T>` replaces exception handling in Rust
- Pattern matching on lists vs slices
- When to use recursion vs iterator methods
- Composing options with `.map()`
- Borrowing strategies for list processing

## OCaml Approach

OCaml uses recursive pattern matching on cons lists. An empty list `[]` returns `None`, and for a cons cell `h::t`, we recursively find the max of the tail and compare it with the head. The Optional type (`'a option`) provides a clean way to signal "no result" without raising.

## Rust Approach

Rust provides idiomatic list operations through slices (`&[T]`) and iterators. The `.max()` method directly solves the problem; pattern matching on slice ranges `[h, rest @ ..]` provides a recursive alternative close to OCaml. Ownership rules ensure safe borrowing without deep copies.

## Key Differences

1. **List Representation:** OCaml uses cons lists (`'a list`); Rust uses slices (`&[T]`) which are more efficient for sequential access.
2. **Max-Finding:** OCaml recurses through the list; Rust uses `.max()` iterator, which is more idiomatic and optimized.
3. **Pattern Matching:** OCaml matches on `[]` and `h::t`; Rust matches on slice patterns like `[h]`, `[h, rest @ ..]`.
4. **Ownership:** Rust borrows the slice, no allocation or copying; OCaml constructs new list nodes during recursion.
5. **Composition:** Both use `map()` to chain operations on Options—a universal functional pattern.

## Testing

Covers empty list, single element, multiple elements, and negative numbers:

```bash
cargo test -p example-1000-option-list-max
```

Expected: 12 tests passing (4 for list_max, 4 for list_max_recursive, 2 for safe_head, 2 for option_map).

## Exercises

1. Write `safe_sum` that returns `None` for an empty list and `Some(total)` otherwise, using only a fold.
2. Implement `safe_max_by` that finds the maximum element according to a comparator closure `f: (&T, &T) -> std::cmp::Ordering`, returning `None` for empty input.
3. Chain several `Option`-returning operations together with `?\/and_then` to parse a comma-separated list of integers, find the max, double it, and return `None` at any step that fails.
