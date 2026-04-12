📖 **[View on hightechmind.io →](https://hightechmind.io/rust/008-eliminate-duplicates)**

---

# Example 008: Eliminate Consecutive Duplicates
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Eliminate consecutive duplicate elements from a list. Only remove duplicates that are adjacent — non-adjacent duplicates remain.

Removing consecutive duplicates is the decompression step for run-length encoding. It also appears in log deduplication (suppress duplicate consecutive log messages), text compression preprocessing, and clean-up of sensor data with stuck values. The key constraint — only consecutive duplicates are removed — makes this fundamentally different from removing all duplicates (which requires a `HashSet`). Run-length decoding produces this exact output.

## Learning Outcomes

- Use `dedup()` for in-place mutation vs building a new collection
- Understand `windows(2)` for pairwise element comparison
- See how Rust's ownership model makes the mutation/immutation choice explicit
- Practice the fold pattern for building filtered results
- Compare OCaml's pattern matching on cons cells with Rust's slice patterns

## OCaml Approach

Pattern matches on the list head, comparing consecutive elements. When `h1 = h2`, skips the duplicate and recurses on the tail. Builds a new list (old one is GC'd).

## Rust Approach

1. **Mutable**: `Vec::dedup()` — in-place, O(n), modifies the vector directly
2. **Functional**: Iterate with a result accumulator, comparing `last()` element
3. **Windows**: Use `windows(2)` to compare pairs, filter where different, collect

## Key Differences

1. **Mutation is explicit**: `dedup()` requires `&mut Vec<T>` — you can't accidentally mutate in Rust
2. **No cons cells**: Rust doesn't have linked-list pattern matching; slices and iterators fill that role
3. **`windows(2)` is unique to Rust**: Efficient pairwise comparison over contiguous memory
4. **Trait bounds**: Rust needs `PartialEq` explicitly; OCaml uses polymorphic equality
5. **In-place vs functional**: Rust naturally offers both; OCaml is functional-first (no in-place dedup on lists)

1. **Mutation vs immutable:** Rust's `dedup()` mutates in place. OCaml lists are immutable — `compress` always builds a new list.
2. **`PartialEq` vs structural equality:** Rust's `dedup` uses `PartialEq` for comparison. OCaml uses structural equality `(=)` by default, which works for most types.
3. **`windows(2)`:** Rust's sliding-window iterator has no built-in OCaml equivalent — it's possible only on contiguous memory (slices/arrays), not linked lists.
4. **Single pass:** All three Rust implementations are single-pass O(n). OCaml's recursive version is also single-pass — it compares the head with the next element.

## Exercises

1. Implement `deduplicate_all` (not just consecutive) — remove every duplicate from a list, keeping only the first occurrence of each element.
2. Write `deduplicate_by` that removes consecutive duplicates using a key function `f: &T -> K` for comparison, so you can deduplicate case-insensitively or by a struct field.
3. Implement `run_length_from_dedup` that uses `eliminate_consecutive` as a building block to produce run-length encoding in a single pipeline (no extra passes).
