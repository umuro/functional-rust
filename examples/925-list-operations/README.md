📖 **[View on hightechmind.io →](https://hightechmind.io/rust/925-list-operations)**

---

# 925-list-operations — List Operations

## Problem Statement

Lists are the fundamental data structure of functional programming. OCaml's standard library is built around singly-linked `list`: head::tail destructuring, recursive definitions, and higher-order functions like `map`, `filter`, and `fold`. Rust uses `Vec<T>` as the workhorse — heap-allocated, contiguous, with O(1) indexed access — but supports functional-style operations through the `Iterator` trait. Learning to express classical list algorithms in Rust illuminates both the idioms of functional programming and the differences between Rust's ownership-based and OCaml's GC-based memory models.

## Learning Outcomes

- Implement `length`, `sum`, `append`, `reverse`, `map`, and `filter` using Rust iterators
- Understand the recursive equivalent of each operation using slice pattern matching
- Recognize the difference between recursive list patterns in OCaml and slice patterns in Rust
- Use `split_first()` as the Rust equivalent of OCaml's `head :: tail` destructuring
- Compare iterator-based and recursive implementations of the same operations

## Rust Application

The code implements two versions of each operation: an idiomatic iterator version (`.iter().sum()`, `.iter().rev().cloned().collect()`) and a recursive slice-pattern version. Recursive `length_recursive` uses `split_first()` to destructure `(&head, tail)` — the closest Rust equivalent to OCaml's `x :: rest`. The recursive versions demonstrate OCaml-style thinking in Rust, while the iterator versions show the idiomatic Rust approach. Both are correct; the iterator versions are more efficient and idiomatic.

## OCaml Approach

OCaml's `List` module provides all these as library functions, each implemented recursively on the singly-linked list structure. `let rec length = function | [] -> 0 | _ :: t -> 1 + length t`. `List.rev_append` is the tail-recursive reverse. OCaml's pattern matching on `x :: xs` is syntactically integrated; Rust's `split_first()` and `[head, rest @ ..]` patterns serve the same purpose but with ownership/borrowing constraints. OCaml lists are persistent and shared via GC; Rust `Vec` is owned and mutable.

## Key Differences

1. **Data structure**: OCaml uses singly-linked immutable lists (prepend O(1), indexed access O(n)); Rust uses `Vec<T>` (append O(1) amortized, indexed access O(1)).
2. **Pattern matching**: OCaml's `x :: xs` destructures a list; Rust uses `split_first()` or `[head, rest @ ..]` slice patterns on `&[T]`.
3. **Sharing**: OCaml lists share tails via GC (prepend creates shared structure); Rust `Vec` clones or moves data.
4. **Tail recursion**: OCaml relies on TCO for stack-safe recursion; Rust iterators eliminate the stack depth concern for production use.

## Exercises

1. Implement `zip_lists<A, B>(a: &[A], b: &[B]) -> Vec<(A, B)>` both iteratively and recursively.
2. Write `flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T>` using both `.iter().flatten()` and a manual recursive approach.
3. Implement `group_consecutive<T: Eq + Clone>(data: &[T]) -> Vec<Vec<T>>` recursively using slice patterns.
