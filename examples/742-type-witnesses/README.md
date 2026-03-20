📖 **[View on hightechmind.io →](https://hightechmind.io/rust/742-type-witnesses)**

---

# 742-type-witnesses — Type Witnesses

## Problem Statement

A type witness is a value that proves an invariant holds at the type level. Rather than re-checking a property every time (is this list sorted? is this range non-empty?), you carry evidence of the property in the type. The `Sorted<T>` witness means "this Vec was produced by sorting, so binary search is safe without re-checking." This pattern eliminates entire categories of defensive checks and makes APIs self-documenting about their preconditions. Used in Haskell's `Data.Map.Strict` and Rust's `BTreeMap`.

## Learning Outcomes

- Implement `Sorted<T>` as a newtype whose only constructor sorts the input
- Provide a `merge` function that preserves the sorted witness through a merge-sort merge step
- Use `binary_search` safely on `Sorted<T>` without runtime re-verification
- Understand how private fields enforce that witnesses cannot be forged
- See how the witness pattern scales to other invariants: `NonEmpty<T>`, `Deduplicated<T>`

## Rust Application

`Sorted<T>(Vec<T>)` has a private inner vec. The only entry point is `Sorted::sort(v: Vec<T>) -> Self`, which calls `v.sort()` before wrapping. `merge` takes two `Sorted<T>` values and returns a `Sorted<T>` — provably sorted because the merge-sort merge of two sorted sequences is sorted. `binary_search` exploits the invariant directly without a defensive sort check. The witness is zero-cost: just a newtype.

## OCaml Approach

OCaml achieves type witnesses via abstract types in modules. A `Sorted` module exposes `type 'a t` with `sort : 'a list -> 'a t` and `merge : 'a t -> 'a t -> 'a t` but hides the constructor. GADTs provide an even stronger mechanism: `type _ sorted = Sorted : 'a list -> 'a sorted` encodes the proof directly in the GADT index. The `Sequence` library uses this for lazy sorted sequences.

## Key Differences

1. **Enforcement mechanism**: Rust uses private fields + crate boundaries; OCaml uses abstract module types or GADTs.
2. **GADT witnesses**: OCaml's GADTs allow witnesses that carry compile-time proofs (e.g., list length); Rust achieves similar guarantees via const generics or type-level tricks.
3. **Merge safety**: Both languages can express "merge of two sorted = sorted" in the type system, though Rust's trait bounds make it more explicit.
4. **Deduplication**: Rust can compose witnesses (`Sorted<Deduplicated<T>>`) via nested newtypes; OCaml uses multiple abstract type aliases.

## Exercises

1. Implement `NonEmpty<T>` as a witness newtype with `first()` and `last()` methods that are infallible (no `Option`), because the non-empty invariant is guaranteed.
2. Create a `Deduplicated<T>` witness and implement `dedup(Sorted<T>) -> Sorted<Deduplicated<T>>` — taking advantage of the fact that dedup is only efficient on sorted input.
3. Write a `merge_k` function that takes `Vec<Sorted<T>>` and merges them all into a single `Sorted<T>` using a min-heap, preserving the sortedness witness.
