📖 **[View on hightechmind.io →](https://hightechmind.io/rust/893-group-by-iter)**

---

# 893-group-by-iter — Group By Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Run-length encoding, log analysis, and data aggregation all need to group consecutive equal elements or elements sharing a common key. SQL's `GROUP BY` aggregates all rows matching a key regardless of position. A consecutive group-by collapses adjacent equal elements — used in run-length encoding, detecting intervals of the same event type, and aggregating time-series data where consecutive observations belong to the same period. Haskell's `Data.List.groupBy` and OCaml's own group pattern both handle consecutive grouping. Rust's standard library has `.chunk_by()` (1.77+); for earlier versions or custom key logic, it must be implemented manually.

## Learning Outcomes

- Implement group-consecutive using a peekable iterator pattern
- Implement group-by-key that produces `(key, group)` pairs
- Use `HashMap` for non-consecutive grouping (all elements sharing a key)
- Compare consecutive grouping (run-length style) with global grouping (SQL style)
- Recognize the difference between `Itertools::group_by` (consecutive) and `HashMap::entry` (global)

## Rust Application

`group_consecutive` uses a peekable-style approach: iterate, compare each element to the current group's first element, flush and start a new group on mismatch. `group_by_key` generalizes this with a key function, producing `Vec<(K, Vec<T>)>` pairs. The count-runs function demonstrates the application: `group_consecutive` followed by `.map(|(k, g)| (k, g.len()))` produces run-length encoding. The HashMap-based `group_by_all` groups non-consecutively: all elements with the same key regardless of position.

## OCaml Approach

OCaml implements group-by recursively: `let rec group_by eq = function | [] -> [] | x :: rest -> let (same, others) = List.partition (eq x) rest in (x :: same) :: group_by eq others`. This is O(n²) for the partition; the efficient consecutive version uses pattern matching on the head. For consecutive grouping: `let rec group_consecutive = function | [] -> [] | [x] -> [[x]] | x :: y :: rest when x = y -> ... `. Libraries like `Base.List.group_by` and `Core.List.group` provide this.

## Key Differences

1. **Consecutive vs global**: Both languages have consecutive grouping (run-length style); global grouping requires a `HashMap` in Rust or `Hashtbl` in OCaml.
2. **Standard library gap**: Rust's `chunk_by` was added in 1.77; before that, it required the `itertools` crate or manual implementation; OCaml requires a library.
3. **Output structure**: Rust produces `Vec<Vec<T>>` or `Vec<(K, Vec<T>)>`; OCaml produces `'a list list` or association lists.
4. **Peekable vs recursive**: Rust uses a peekable loop for efficiency; OCaml uses recursive pattern matching.

## Exercises

1. Implement `run_length_encode<T: Eq + Clone>(data: &[T]) -> Vec<(T, usize)>` using `group_consecutive`.
2. Write `group_by_all<T: Clone, K: Eq + Hash>(data: &[T], key: impl Fn(&T) -> K) -> HashMap<K, Vec<T>>` for non-consecutive grouping.
3. Implement `group_by_ranges(data: &[i32], range_size: i32) -> Vec<(i32, Vec<i32>)>` that groups numbers into buckets of size `range_size`.
