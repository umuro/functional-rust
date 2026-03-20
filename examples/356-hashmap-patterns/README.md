📖 **[View on hightechmind.io →](https://hightechmind.io/rust/356-hashmap-patterns)**

---

# 356: HashMap Patterns

## Problem Statement

Counting, grouping, and frequency analysis are among the most common data processing tasks. A hash map is the universal tool: O(1) average insert and lookup, with keys mapped to values. Rust's `HashMap<K, V>` uses the SipHash-1-3 algorithm by default (DoS-resistant), supports the `entry()` API for atomic insert-or-update, and requires `K: Hash + Eq`. Understanding the three canonical patterns — word counting, grouping by key, and top-N frequency — prepares you for most real-world data pipeline work.

## Learning Outcomes

- Build a word frequency counter using `entry().or_insert(0)` and `+= 1`
- Implement `group_by<T, K>` that partitions items into a `HashMap<K, Vec<T>>`
- Extract top-N entries by sorting pairs by value descending
- Understand that `entry()` avoids double-lookup vs `get` + `insert`
- Use `or_default()` as a shorthand for `or_insert(Default::default())`
- Recognize `HashMap`'s O(1) average vs `BTreeMap`'s O(log n) sorted operations

## Rust Application

```rust
use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

pub fn group_by<T, K, F>(items: Vec<T>, key: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        map.entry(key(&item)).or_default().push(item);
    }
    map
}

pub fn frequency_top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut pairs: Vec<_> = map.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1)); // descending by count
    pairs.into_iter().take(n).collect()
}
```

The `group_by` pattern generalizes to any classification: grouping transactions by date, files by extension, users by country. `or_default()` works when `V: Default` (like `Vec<T>`, `usize`, `String`), sparing you from writing `or_insert_with(Vec::new)`.

## OCaml Approach

OCaml's `Hashtbl` provides the imperative equivalent:

```ocaml
let word_count text =
  let tbl = Hashtbl.create 64 in
  String.split_on_char ' ' text |> List.iter (fun w ->
    let count = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (count + 1));
  tbl

(* Functional alternative with Map *)
module SMap = Map.Make(String)
let word_count_pure text =
  String.split_on_char ' ' text
  |> List.fold_left (fun m w ->
    let n = try SMap.find w m with Not_found -> 0 in
    SMap.add w (n + 1) m) SMap.empty
```

The `Hashtbl` version mutates in place; the `Map` version returns a new map per insertion (persistent). Rust's `HashMap` is imperative like `Hashtbl`, but the entry API makes the "find or insert" pattern atomic and idiomatic.

## Key Differences

| Aspect | Rust `HashMap` | OCaml `Hashtbl` |
|--------|---------------|-----------------|
| Insert-or-update | `entry().or_insert()` | `find` + `replace` (two lookups) |
| Hash algorithm | SipHash-1-3 (DoS-resistant) | Polymorphic hash (faster, not DoS-safe) |
| Ordering | None | None |
| `group_by` | `or_default().push(item)` | `find` + `replace` with list cons |
| Alternative | `BTreeMap` for ordered | `Map.Make` for ordered |

## Exercises

1. **Anagram grouping**: Given a list of words, group anagrams together: `group_by(words, |w| { let mut sorted = w.chars().collect::<Vec<_>>(); sorted.sort(); sorted })`; output groups where each group contains words that are anagrams of each other.
2. **Inverted index**: Build an inverted index mapping each word to the set of document IDs it appears in, using `HashMap<String, HashSet<usize>>`; then query "documents containing both 'rust' and 'async'".
3. **LRU approximation**: Implement a simple frequency-based cache using `HashMap<K, (V, usize)>` tracking access counts; evict the least-frequently-used entry when size exceeds capacity.
