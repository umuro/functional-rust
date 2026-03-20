📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1031-hashmap-counting)**

---

# 1031-hashmap-counting — Frequency Counting with HashMap

## Problem Statement

Frequency counting is the foundation of histogram generation, text analysis, log aggregation, and A/B test result collection. The operation reads each item from a stream and increments a counter in a map, requiring an efficient insert-or-increment primitive.

Rust's `Entry` API provides `and_modify(|c| *c += 1).or_insert(1)` as the canonical single-lookup pattern. This example explores all the Entry API variants for counting, showing how each is optimized for different use cases.

## Learning Outcomes

- Count character frequencies with `entry(ch).or_insert(0)` and `*count += 1`
- Use `and_modify(|c| *c += 1).or_insert(1)` for combined increment-or-insert
- Find the most frequent element after counting
- Use `HashMap::iter()` to iterate and sort counts
- Understand `entry` vs re-lookup performance characteristics

## Rust Application

`src/lib.rs` shows `char_frequency` using the two-statement `or_insert(0)` + `*= 1` pattern, and `word_frequency` using the chainable `and_modify(...).or_insert(1)` form. `most_frequent` finds the maximum count by iterating over the map and using `max_by_key`. The `and_modify` form is slightly more readable for the increment case because it chains semantically: "if present, modify; if absent, insert."

This exact pattern appears in `serde`'s deserialization visitors, in `rayon`'s parallel reduce, and in virtually every Rust data pipeline.

## OCaml Approach

OCaml uses `Hashtbl` for mutable counting or `Map` for immutable:

```ocaml
let char_frequency s =
  let tbl = Hashtbl.create 16 in
  String.iter (fun c ->
    let count = try Hashtbl.find tbl c with Not_found -> 0 in
    Hashtbl.replace tbl c (count + 1)
  ) s;
  tbl
```

`Base.Hashtbl.incr` provides a one-liner: `Hashtbl.incr tbl ~key:c ~by:1 ~default:0`.

## Key Differences

1. **Single lookup**: Rust's `entry().and_modify().or_insert()` is one hash computation; OCaml's `find + replace` is two.
2. **Chained API**: Rust's `and_modify(...).or_insert(1)` is a fluent chain; OCaml's stdlib requires two statements.
3. **`Base.Hashtbl.incr`**: OCaml's `Base` library provides a dedicated increment function; Rust's stdlib does not but the Entry API is equivalent.
4. **Return value**: Rust's `or_insert` returns `&mut V` enabling direct mutation; OCaml's `Hashtbl.find` returns the value by copy.

## Exercises

1. Write `top_k_chars(text: &str, k: usize) -> Vec<(char, usize)>` that returns the k most frequent characters sorted by descending count.
2. Implement a streaming counter `FrequencyCounter<K>` struct with `add(&mut self, key: K)` and `top_n(n: usize) -> Vec<(K, usize)>` methods.
3. Write a function that detects the first character that appears exactly once in a string using the frequency map.
