# Example 1218: List Sort — Sort with Custom Comparator

**Difficulty:** ⭐⭐
**Category:** Lists & HOF
**OCaml Source:** `List.sort` — OCaml standard library (`Stdlib.List`)

## Problem Statement

Given a list of strings (e.g. `["banana"; "apple"; "cherry"; "date"]`),
produce new lists sorted (a) alphabetically and (b) by length, using a
caller-supplied comparator. The original list must not be mutated.

## Learning Outcomes

- Translating `List.sort : ('a -> 'a -> int) -> 'a list -> 'a list` into Rust's slice sort family (`sort_by`, `sort_by_key`).
- How OCaml's `int`-returning comparator becomes Rust's `Ordering` enum (`Less` / `Equal` / `Greater`).
- Why `sort_by_key` is the preferred form when the comparator is pure key extraction — it is `O(n)` keys + `O(n log n)` comparisons, not `O(n log n)` key recomputations.
- Stability: both `List.sort` (OCaml) and `slice::sort_by` (Rust) are stable — a tie-break test pins this down.
- Returning a **new** `Vec<T>` (via `to_vec()` + in-place sort) to mirror OCaml's non-mutating semantics.

## OCaml Approach

`List.sort` takes a three-way comparator returning a negative / zero /
positive `int` and returns a brand-new sorted list, leaving the input
untouched. Common comparators (`String.compare`, `compare`) are already
three-way functions, so they can be passed by name. Custom orderings are
built as lambdas: `fun a b -> compare (f a) (f b)`.

## Rust Approach

Rust's sort family lives on slices and sorts **in place**. To mirror
OCaml's "returns a new list" contract, we `clone`/`to_vec` the input
first, then call `sort_by` (for a full comparator) or `sort_by_key` (for
key-based comparators — the idiomatic shortcut). The closure returns a
`std::cmp::Ordering` value built by chaining `.cmp()` / `.then_with()`.

## Key Differences

1. **Return type of the comparator:** OCaml uses `int` (sign matters, magnitude is ignored); Rust uses the `Ordering` enum — type-safe but less flexible when subtracting numeric keys.
2. **In-place vs pure:** OCaml's `List.sort` is pure (returns a new list); Rust's `sort_by` mutates an owned `Vec`. You pay one `clone` per call to preserve the OCaml contract.
3. **Key extraction shortcut:** Rust has `sort_by_key` / `sort_by_cached_key` — OCaml has no direct equivalent and re-extracts the key inside the lambda every comparison.
4. **Stability:** both are stable. Rust's `sort_unstable_by` is faster but loses the tie-break guarantee — read the method name carefully.
