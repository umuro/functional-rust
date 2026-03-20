📖 **[View on hightechmind.io →](https://hightechmind.io/rust/090-frequency-analysis)**

---

# Example 090: Frequency Analysis — Letter Distribution

**Difficulty:** ⭐⭐
**Category:** String Processing
**OCaml Source:** Classic functional programming — character frequency with ordered maps

## Problem Statement

Count how many times each letter (a–z) appears in a string, ignoring case and non-alphabetic characters, then return results sorted by frequency descending.

## Learning Outcomes

- How `HashMap::entry` with `or_insert` replaces OCaml's `Map.update` for in-place counting
- Why `BTreeMap` mirrors OCaml's `Map.Make(Char)` — both keep keys sorted, both are O(log n) per operation
- How iterator chains (`.filter().map().fold()`) replace OCaml's `String.fold_left` pipeline
- The difference between hash-based (`HashMap`) and tree-based (`BTreeMap`) maps and when to choose each

## OCaml Approach

OCaml uses a functor `Map.Make(Char)` to create a balanced BST map keyed on `char`. The `String.fold_left` threads a map accumulator through each character, calling `CMap.update` to increment or initialize counts. The map is then converted to a sorted binding list for display.

## Rust Approach

Rust offers two natural equivalents: `HashMap<char, usize>` for O(1) average access and `BTreeMap<char, usize>` for O(log n) with keys in sorted order — the latter directly mirrors OCaml's `Map.Make`. The `entry().or_insert(0)` pattern compresses OCaml's `update ... function None -> Some 1 | Some n -> Some (n+1)` into a single idiomatic expression.

## Key Differences

1. **Map type:** OCaml uses `Map.Make(Char)` (balanced BST, sorted by key); Rust offers `HashMap` (hash table, unordered) or `BTreeMap` (B-tree, sorted by key)
2. **Update idiom:** OCaml's `CMap.update c f m` takes a function over `option`; Rust's `map.entry(c).or_insert(0)` returns a mutable reference for in-place mutation
3. **String iteration:** OCaml's `String.fold_left` threads state; Rust's `.chars().filter().map().fold()` composes the same pipeline with explicit types
4. **Sorting:** OCaml's `List.sort (fun (_, a) (_, b) -> compare b a)` sorts by value descending; Rust's `.sort_by(|(c1,n1),(c2,n2)| n2.cmp(n1).then(c1.cmp(c2)))` adds a stable tiebreaker

## Exercises

1. Extend frequency analysis to produce a normalized frequency map (`HashMap<char, f64>`) where values sum to 1.0, then compute the cosine similarity between two texts.
2. Implement `index_of_coincidence` — the probability that two randomly selected letters from a text are the same — and use it to estimate whether a text is in English.
3. Build a Vigenère cipher breaker: use the index of coincidence to guess the key length, then apply frequency analysis on each substream to recover the key.
