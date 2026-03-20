📖 **[View on hightechmind.io →](https://hightechmind.io/rust/278-iterator-last)**

---

# 278: Getting the Last Element with last()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Retrieving the final element of a sequence is straightforward for slices (`slice.last()`) but requires consuming the entire iterator for non-indexed collections. The `last()` method provides a safe, idiomatic way to get the final element of any iterator, returning `Option<T>` to handle the empty case. Combined with `filter()`, it finds the last matching element in a single pass.

## Learning Outcomes

- Understand that `last()` must consume the entire iterator to find the final element
- Use `last()` after `filter()` to find the last matching element
- Recognize that `last()` on a `DoubleEndedIterator` (like slice iterators) can optimize to O(1)
- Distinguish slice `last()` from iterator `last()` — same semantics, different implementations

## Rust Application

`Iterator::last()` consumes the iterator and returns `Option<T>`. For iterators that also implement `DoubleEndedIterator`, the standard library can optimize this to a single `next_back()` call:

```rust
let v = [1i32, 2, 3, 4, 5];
assert_eq!(v.iter().last(), Some(&5));

let empty: Vec<i32> = vec![];
assert_eq!(empty.iter().last(), None);

// Last even number
let last_even = (1i32..=10).filter(|x| x % 2 == 0).last();
assert_eq!(last_even, Some(10));
```

## OCaml Approach

OCaml's standard library provides `List.rev` then `List.hd` for the last element, though this allocates a reversed list. More efficient is a fold:

```ocaml
let last = function [] -> None | xs -> Some (List.nth xs (List.length xs - 1))
(* Or more efficiently: *)
let last lst = List.fold_left (fun _ x -> Some x) None lst
```

`List.hd (List.rev lst)` is common but allocates; the fold approach is O(n) without allocation.

## Key Differences

1. **O(n) cost**: Both Rust and OCaml must consume the full iterator/list to find the last element of a forward-only structure.
2. **DoubleEnded optimization**: Rust's `last()` on `DoubleEndedIterator` can call `next_back()` for O(1) access; OCaml has no equivalent built-in optimization.
3. **Memory**: Rust's `last()` keeps only one element in memory at a time; OCaml's `List.rev` allocates a full reversed list.
4. **Composability**: `filter().last()` is the clean Rust idiom; OCaml requires `List.filter` + last — two separate operations.

## Exercises

1. Find the last line in a log file (simulated as a `Vec<String>`) that matches a warning pattern.
2. Implement a function that returns the last N elements of an iterator without materializing the full iterator — use a `VecDeque` as a sliding window.
3. Verify that `last()` and `fold(None, |_, x| Some(x))` produce identical results.
