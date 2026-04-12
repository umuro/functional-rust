📖 **[View on hightechmind.io →](https://hightechmind.io/rust/917-iterator-last)**

---

# 917-iterator-last — Iterator Last
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Retrieving the final element of a sequence has different costs depending on the data structure: O(1) for arrays and `Vec` (by index), O(n) for linked lists (must traverse all). For iterators, `.last()` always traverses the entire sequence — it must consume every element to reach the end. This makes `.last()` unsuitable for large sequences where only the final element is needed; a `DoubleEndedIterator` with `.next_back()` is O(1) for slices. Understanding this cost difference helps write efficient code. OCaml has `List.rev xs |> List.hd` which is similarly O(n).

## Learning Outcomes

- Use `.last()` to retrieve the final element of an iterator
- Understand that `.last()` consumes the entire iterator — O(n) always
- Use `DoubleEndedIterator::next_back()` as O(1) alternative for slices
- Apply `.last()` after filtering or mapping to get the last matching element
- Compare with OCaml's `List.rev |> List.hd` and array back-indexing

## Rust Application

The tests show: `[1,2,3,4,5].iter().last()` = `Some(&5)`. Empty iterator: `last()` = `None`. After filter: `(1i32..=10).filter(|x| x % 2 == 0).last()` = `Some(10)`. Single element: `[42].iter().last()` = `Some(&42)`. For slices, prefer `data.last()` (O(1) via `DoubleEndedIterator`) over `data.iter().last()` (O(n)). The distinction matters for large vectors where the final element is the goal without any transformation.

## OCaml Approach

`List.rev xs |> List.hd` is the standard but inefficient approach — O(n) reversal then O(1) head. `let rec last = function | [] -> None | [x] -> Some x | _ :: rest -> last rest` is O(n) with O(1) allocation. `Array.get arr (Array.length arr - 1)` is O(1) for arrays. OCaml lacks a standard `List.last` in the standard library (it is in `Base.List.last`).

## Key Differences

1. **Option vs exception**: Rust `.last()` returns `Option<T>` — `None` for empty; OCaml `List.hd` raises on empty list (`List.last_opt` from `Base` is safe).
2. **O(n) always**: Both Rust `.last()` on iterators and OCaml `List.last` are O(n); array/slice back-access is O(1) in both languages.
3. **Standard library**: Rust has `.last()` on all iterators; OCaml standard library lacks `List.last` (use `Base` or recursion).
4. **DoubleEnded alternative**: Rust's `DoubleEndedIterator::next_back()` provides O(1) last-element access for slices and many other iterators.

## Exercises

1. Implement `second_to_last<T>(data: &[T]) -> Option<&T>` without reversing the slice.
2. Write `last_n<T: Clone>(iter: impl Iterator<Item=T>, n: usize) -> Vec<T>` that collects the last n elements without knowing the total count in advance.
3. Implement `last_matching<T>(data: &[T], pred: impl Fn(&T) -> bool) -> Option<&T>` using a reverse iterator for O(1) discovery when the match is near the end.
