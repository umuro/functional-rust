📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1043-interval-map)**

---

# 1043-interval-map — Interval Map

## Problem Statement

An interval map assigns values to non-overlapping ranges of a key space. Use cases: IP address routing tables (CIDR blocks), calendar scheduling (time ranges), tax brackets (income ranges), and memory segment descriptors in operating systems.

The canonical implementation uses a sorted `BTreeMap<start, (end, value)>` where each entry represents one interval. Lookup requires finding the largest start key that is ≤ the query point and verifying the end bound, which is O(log n) with `BTreeMap::range`.

## Learning Outcomes

- Implement an interval map using `BTreeMap<i64, (i64, V)>`
- Handle interval insertion with overlap removal
- Perform O(log n) point queries
- Understand the design of sorted-key interval structures
- Connect this to `std::collections::BTreeMap::range` API

## Rust Application

`src/lib.rs` implements `IntervalMap<V>` with `map: BTreeMap<i64, (i64, V)>`. `insert(lo, hi, value)` first removes all overlapping intervals by scanning `map.range(..hi)` for entries whose end is > lo. `query(point)` uses `map.range(..=point)` to find the candidate interval (largest start ≤ point) and checks that point < end.

Interval maps appear in `nodit` crate, Linux kernel's memory management (`mmap` ranges), and network routing table implementations.

## OCaml Approach

OCaml's `Map.Make` enables the same approach:

```ocaml
module IntMap = Map.Make(Int)

type 'v interval_map = (int * 'v) IntMap.t  (* key=lo, value=(hi, v) *)

let query m point =
  match IntMap.find_last_opt (fun lo -> lo <= point) m with
  | None -> None
  | Some (lo, (hi, v)) -> if point < hi then Some v else None
```

`Map.find_last_opt` finds the largest key satisfying a predicate — the equivalent of Rust's `range(..=point).next_back()`.

## Key Differences

1. **Range queries**: Rust uses `map.range(..=point).next_back()` to find the predecessor; OCaml uses `Map.find_last_opt (fun k -> k <= point)`.
2. **Overlap removal**: Both use range iteration to find overlapping intervals, but Rust's `BTreeMap::range` is O(k + log n) where k is the number of overlapping intervals.
3. **Mutability**: Rust's `BTreeMap` is mutable (intervals removed in place); OCaml returns new map versions.
4. **Production libraries**: The `nodit` crate provides a production-grade interval map with richer APIs; OCaml's `Interval_map` library provides similar functionality.

## Exercises

1. Add a `gaps(lo: i64, hi: i64) -> Vec<(i64, i64)>` method that returns all uncovered sub-intervals within the query range.
2. Implement an IP routing table using `IntervalMap<String>` with integer representations of IPv4 addresses and CIDR blocks.
3. Write a `merge(other: IntervalMap<V>)` method that inserts all intervals from another map, resolving overlaps.
