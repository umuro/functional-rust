📖 **[View on hightechmind.io →](https://hightechmind.io/rust/028-sort-by-length)**

---

# 028 — Sorting a List of Lists According to Length

## Problem Statement

Sorting a list of lists by the length of each sublist (OCaml 99 Problems #28) demonstrates custom comparators — one of the most practical uses of higher-order functions. The problem has two parts: sort by length directly, and sort by length frequency (shorter lists whose length appears more often come first).

Custom comparators are everywhere: sorting database records by multiple fields, sorting search results by relevance score, ordering tasks by priority, and comparing compound keys in B-trees. The functional approach passes a comparator function to the sort algorithm, keeping sort logic separate from comparison logic.

## Learning Outcomes

- Use `sort_by_key` and `sort_by` with custom comparison functions
- Compute length frequency using a `HashMap<usize, usize>` for the secondary sort
- Apply stable sorting to preserve relative order of equal-length lists
- Compare the functional (sort_by_key) and imperative (Schwartzian transform) approaches
- Understand why frequency-based sorting requires a two-pass algorithm

## Rust Application

Part 1 (sort by length): `lists.sort_by_key(|l| l.len())`. Part 2 (sort by frequency): first compute `freq: HashMap<usize, usize>` counting how often each length appears, then `lists.sort_by_key(|l| freq[&l.len()])`. Both use stable sort (Rust's `sort_by_key` is stable). The Schwartzian transform pre-computes keys to avoid recomputing during sort: `let mut tagged: Vec<(usize, &Vec<i32>)> = lists.iter().map(|l| (l.len(), l)).collect(); tagged.sort_by_key(|&(len, _)| len)`.

## OCaml Approach

OCaml's version: `List.sort (fun a b -> compare (List.length a) (List.length b)) lists`. For frequency sort: compute a frequency map, then `List.sort (fun a b -> compare (freq (List.length a)) (freq (List.length b))) lists`. OCaml's `List.sort` takes a comparison function `'a -> 'a -> int` where negative means "less than". This is the standard compare-function interface used in C's `qsort` and Java's `Comparator`.

## Key Differences

1. **Comparator interface**: OCaml uses `int`-returning comparators (negative/zero/positive). Rust uses `Ordering` enum (`Less`/`Equal`/`Greater`) with `sort_by`, or key functions with `sort_by_key`.
2. **`sort_by_key` vs `sort_by`**: Rust's `sort_by_key(f)` calls `f` once per element before sorting (Schwartzian transform under the hood). `sort_by(cmp)` calls `cmp` O(n log n) times. OCaml's `List.sort` calls the comparator O(n log n) times.
3. **Stability**: Rust's `sort_by_key` is stable. OCaml's `List.sort` is also stable. Both preserve relative order of equal elements.
4. **In-place vs functional**: Rust's `sort_by_key` mutates the `Vec`. OCaml's `List.sort` returns a new sorted list.

## Exercises

1. **Multi-level sort**: Sort a `Vec<Vec<i32>>` first by length, then lexicographically within equal-length groups. Use `sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)))`.
2. **Reverse sort**: Write `sort_by_length_desc(lists: &mut Vec<Vec<i32>>)` that sorts longest-first using `sort_by_key(|l| std::cmp::Reverse(l.len()))`.
3. **Topological sort by frequency**: Given a text document, split into words, group by first letter, and sort those groups by frequency of the first letter across the whole document.
