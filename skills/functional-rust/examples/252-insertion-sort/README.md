# 252: Insertion Sort

**Difficulty:** 1  **Level:** Beginner

Build a sorted list by inserting each element into its correct position in an already-sorted accumulator.

## The Problem This Solves

Insertion sort is the algorithm you naturally use when sorting a hand of playing cards: you pick up one card at a time and slide it left until it's in the right place. It's simple to understand, stable (equal elements keep their original order), and efficient for nearly-sorted data.

Beyond the algorithm itself, this example is a tour of how OCaml's functional patterns translate to Rust. OCaml's `List.fold_left` becomes Rust's `.fold()`. OCaml's `h :: t` list destructuring becomes Rust's `[h, rest @ ..]` slice patterns. OCaml's linear scan for insertion position becomes Rust's `partition_point` binary search — same semantics, better performance.

The example shows three styles: idiomatic in-place Rust (fastest, zero allocation), functional fold (mirrors OCaml), and recursive (word-for-word translation of OCaml's `insert` function). Comparing them reveals what Rust's ownership model adds.

## The Intuition

Start with an empty sorted list. Take the first element from the unsorted input and insert it — into an empty list, it goes first. Take the second element and find the right spot: scan left-to-right, stop when you find a larger element, insert there. Repeat until all elements are placed.

The insert step is the core: given a sorted list and a new element `x`, find the first position where the existing element `h >= x`, and slot `x` in just before `h`. If `x` is larger than everything, it goes at the end.

OCaml's `insert` function does this recursively: if `x <= head`, prepend `x`; otherwise keep the head and recurse into the tail. Rust can express this as slice pattern matching — `[h, rest @ ..]` is exactly `h :: rest`.

## How It Works in Rust

```rust
// Style 1: Idiomatic — in-place, zero allocation
pub fn insertion_sort_inplace<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);  // bubble left until in place
            j -= 1;
        }
    }
}

// Style 2: Functional — mirrors OCaml's fold structure
pub fn insertion_sort_functional<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter().cloned().fold(Vec::new(), |mut acc, x| {
        let pos = acc.partition_point(|h| h < &x); // binary search for insert pos
        acc.insert(pos, x);
        acc
    })
}

// Style 3: Recursive — direct translation of OCaml's `insert`
pub fn insert_rec<T: Ord + Clone>(x: T, list: &[T]) -> Vec<T> {
    match list {
        [] => vec![x],
        [h, rest @ ..] => {
            if x <= *h {
                let mut result = vec![x];  // x goes before h
                result.extend_from_slice(list);
                result
            } else {
                let mut result = vec![h.clone()];
                result.extend(insert_rec(x, rest)); // recurse into tail
                result
            }
        }
    }
}
```

## What This Unlocks

- **Slice pattern matching** — `[h, rest @ ..]` is idiomatic Rust for head/tail decomposition; use it in parsers, recursive algorithms, and data processing.
- **`partition_point` as a search primitive** — binary search for insertion position; applicable anywhere you maintain a sorted `Vec`.
- **Fold for accumulation** — the `.fold(Vec::new(), |acc, x| ...)` pattern builds any accumulator from an iterator; generalises far beyond sorting.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| List head access | `h :: t` (cons pattern) | `[h, rest @ ..]` (slice pattern) |
| Fold accumulation | `List.fold_left (fun acc x -> ...) []` | `.fold(Vec::new(), \|acc, x\| ...)` |
| Insert position | Linear scan: `if x <= h` | `partition_point` binary search — O(log n) |
| In-place sorting | Not idiomatic (immutable lists) | `slice::swap` — zero allocation |
| Stability | Stable (`x <= h` preserves order) | Stable (same condition) |
