📖 **[View on hightechmind.io →](https://hightechmind.io/rust/352-btreeset-sorted)**

---

# 352: BTreeSet Sorted

## Problem Statement

When you need a set that maintains elements in sorted order — for range membership tests, sorted output, or ordered iteration — `BTreeSet` is the right choice over `HashSet`. Like `BTreeMap`, it uses a B-tree internally, giving O(log n) insert/remove/contains and guaranteed sorted iteration. This is the structure behind sorted unique element collections in databases (sorted indexes), lexicographic ordering for string sets, and anywhere you need both deduplication and sorted traversal without re-sorting after each insertion.

## Learning Outcomes

- Construct a `BTreeSet<T>` from a `Vec<T>` (automatically deduplicates and sorts)
- Use set operations: `union`, `intersection`, `difference`, `symmetric_difference`
- Iterate in sorted order without explicit sorting
- Use `.range(lo..=hi)` for efficient range membership queries
- Understand that `BTreeSet<T>` requires `T: Ord` (not just `Hash + Eq`)
- Recognize when `BTreeSet` beats `HashSet` (sorted output, range queries)

## Rust Application

```rust
use std::collections::BTreeSet;

pub fn sorted_set<T: Ord>(items: Vec<T>) -> BTreeSet<T> {
    items.into_iter().collect() // duplicates removed, sorted
}

pub fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}

pub fn intersection<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.intersection(b).cloned().collect()
}

pub fn difference<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.difference(b).cloned().collect() // elements in a but not b
}
```

All set operations return iterators (not `BTreeSet` directly) — `.cloned().collect()` materializes them. Since both input sets are sorted, all operations run in O(n + m) — a merge-like scan, not a hash-based lookup loop. The output is also sorted.

## OCaml Approach

OCaml's `Set.Make` functor creates ordered sets:

```ocaml
module IntSet = Set.Make(Int)

let sorted_set items =
  List.fold_left (fun s x -> IntSet.add x s) IntSet.empty items

let union a b = IntSet.union a b
let inter a b  = IntSet.inter a b
let diff a b   = IntSet.diff a b
```

OCaml's `Set` is a purely functional AVL tree — operations return new sets. `Set.union`, `Set.inter`, `Set.diff` all run in O(m log(n/m)) for sets of size n and m. Like Rust's `BTreeSet`, iteration is always sorted.

## Key Differences

| Aspect | Rust `BTreeSet` | OCaml `Set.Make` |
|--------|----------------|------------------|
| Mutability | In-place (`insert`, `remove`) | Persistent (new set returned) |
| Set operations | Return iterator, collect to set | Return new set directly |
| Ordering | `T: Ord` (total order) | `compare` function via functor |
| Range query | `.range(lo..=hi)` iterator | `filter` (O(n)) or split then traverse |
| Deduplication | On insert | On insert |

## Exercises

1. **Sorted unique words**: Read a list of words, lowercase them, and collect into a `BTreeSet<String>`; verify the output is deduplicated and alphabetically sorted.
2. **Set difference chain**: Given three sets A, B, C, compute elements that are in A but not in B or C using chained `difference` calls; compare with `a.iter().filter(|x| !b.contains(x) && !c.contains(x))`.
3. **Range member test**: Given a `BTreeSet<i32>` of reserved port numbers, write `is_port_range_free(set, start, end) -> bool` using `.range(start..=end).next().is_none()` — O(log n) instead of scanning.
