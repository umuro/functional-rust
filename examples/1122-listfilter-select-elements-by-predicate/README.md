# Example 1122: List Filter — Select Elements by Predicate
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Select elements from a list that satisfy a boolean predicate, equivalent to OCaml's `List.filter`. Demonstrated by splitting a list of integers into even and odd subsets.

## Learning Outcomes

- How OCaml's `List.filter` maps to Rust's `.iter().filter().collect()`
- How `Iterator::partition` computes both halves in one pass — more efficient than filtering twice
- How slice pattern matching (`[head, tail @ ..]`) enables recursive list processing
- The distinction between borrowing (`&[T]`) and owning (`Vec<T>`) in filter operations

## OCaml Approach

OCaml's `List.filter pred lst` traverses the list and builds a new list of elements satisfying `pred`. It is defined recursively in terms of cons (`::`) and is a classic higher-order function. OCaml also has `List.partition` for splitting into two lists at once.

## Rust Approach

Rust's iterator chain `list.iter().filter(|x| pred(x)).cloned().collect()` mirrors `List.filter` directly. For computing both halves, `Iterator::partition` is more efficient than two filter passes. Recursive slice pattern matching (`[head, tail @ ..]`) makes the OCaml structural recursion explicit in Rust.

## Key Differences

1. **List vs Slice:** OCaml uses linked-list `'a list`; Rust uses contiguous `&[T]` slices — no allocation for the input.
2. **Closure reference:** OCaml predicates take values; Rust closures here take `&T` to avoid cloning the input during filtering.
3. **Partition as an optimization:** OCaml computes `evens` and `odds` with two separate `List.filter` calls (two passes); Rust's `partition` does it in one pass.
4. **Clone on output:** OCaml's GC handles sharing; Rust must `.clone()` elements when going from borrowed `&T` to owned `Vec<T>`.

## Exercises

1. Combine `filter` with `map` in a single pass: implement `filter_map` that applies a `T -> Option<U>` function and collects only `Some` results.
2. Implement `take_while` from scratch: return elements from the front of the list as long as the predicate holds, stopping at the first failure.
3. Write `filter_with_context` that passes both the current element and the previously accepted element (as an `Option`) to the predicate, enabling stateful filtering like keeping only elements greater than the last accepted one.
