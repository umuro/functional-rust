📖 **[View on hightechmind.io →](https://hightechmind.io/rust/008-eliminate-duplicates)**

---

# Example 008: Eliminate Consecutive Duplicates
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Eliminate consecutive duplicate elements from a list. Only remove duplicates that are adjacent — non-adjacent duplicates remain.

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

## Exercises

1. Implement `deduplicate_all` (not just consecutive) — remove every duplicate from a list, keeping only the first occurrence of each element.
2. Write `deduplicate_by` that removes consecutive duplicates using a key function `f: &T -> K` for comparison, so you can deduplicate case-insensitively or by a struct field.
3. Implement `run_length_from_dedup` that uses `eliminate_consecutive` as a building block to produce run-length encoding in a single pipeline (no extra passes).
