📖 **[View on hightechmind.io →](https://hightechmind.io/rust/276-parallel-letter-frequency)**

---

# Example 276: Parallel Letter Frequency

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** https://exercism.org/tracks/ocaml/exercises/parallel-letter-frequency

## Problem Statement

Count the frequency of each letter across multiple texts using a map-reduce pattern: map each text to a frequency table, then reduce (merge) all tables into one combined result.

## Learning Outcomes

- How to implement map-reduce with iterators and `fold`
- Using `HashMap::entry` API for efficient in-place updates
- Translating OCaml's `Map.Make` functor to Rust's `HashMap`
- Pattern matching on slices for recursive decomposition

## OCaml Approach

OCaml uses a functor-generated `Map.Make(Char)` for an ordered char map. `String.fold_left` builds per-text frequency maps, and `CMap.union` with a merge function combines them. The pipeline `|> List.map |> List.fold_left` is classic map-reduce.

## Rust Approach

Rust uses `HashMap<char, usize>` with the `entry` API for ergonomic insert-or-update. The iterator chain `.iter().map().fold()` mirrors OCaml's pipeline. A recursive variant uses slice pattern matching `[head, rest @ ..]`.

## Key Differences

1. **Map type:** OCaml uses `Map.Make(Char)` (ordered, functor-generated); Rust uses `HashMap` (unordered, generic)
2. **Entry API:** OCaml's `CMap.update` takes an `option -> option` function; Rust's `entry().or_insert()` is more ergonomic for counters
3. **Merge strategy:** OCaml's `CMap.union` takes a 3-argument merge function; Rust requires manual iteration over entries
4. **Mutability:** OCaml maps are immutable (each operation returns new map); Rust's `HashMap` is mutated in place for efficiency

## Exercises

1. Extend the parallel frequency counter to handle Unicode code points rather than ASCII bytes, using `char` as the key type.
2. Benchmark the parallel version against the sequential version for texts of 1 KB, 100 KB, and 10 MB, and explain at what size parallelism starts to pay off.
3. Generalize the parallel aggregation to a generic `parallel_fold` function that splits any `Vec` into chunks, processes them in parallel, and combines results using a monoid — then use it to compute both letter frequency and total word count in one pass.
