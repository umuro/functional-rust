📖 **[View on hightechmind.io →](https://hightechmind.io/rust/219-hylomorphism)**

---

# Hylomorphism — Ana then Cata, Fused
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A hylomorphism (`hylo`) is the composition of an anamorphism followed by a catamorphism: unfold a seed into a structure, then immediately fold the structure. The profound optimization: the intermediate structure need never be materialized. `hylo` fuses the unfold and fold into a single pass, eliminating intermediate allocation. Mergesort is a canonical hylomorphism: unfold the array into a tree, fold the tree by merging sorted sublists.

## Learning Outcomes

- Understand hylomorphisms as fused `ana` + `cata` with no intermediate structure
- Learn the fusion optimization that eliminates intermediate allocation
- See mergesort as the canonical hylomorphism example
- Understand how `hylo` generalizes `fold_left` (which is a hylomorphism over the natural numbers)

## Rust Application

`hylo<S, A>(coalg: impl Fn(S) -> ListF<S>, alg: impl Fn(ListF<A>) -> A) -> impl Fn(S) -> A` is the fused implementation. The key: instead of building `Fix<ListF>` first and then folding, `hylo` calls `alg(coalg(seed).map(|s| hylo(coalg, alg)(s)))` — the recursive call passes the new seeds directly to the coalgebra. Mergesort: `split_coalg` unfolds into halves, `merge_alg` folds by merging — no intermediate sorted-sublists tree is stored.

## OCaml Approach

OCaml's `hylo`:
```ocaml
let rec hylo coalg alg seed =
  alg (map_list_f (hylo coalg alg) (coalg seed))
```
This is the fused version — `coalg` produces the shape, `map_list_f (hylo coalg alg)` recursively processes children, `alg` combines. OCaml's `let rec` handles the mutual recursion naturally. OCaml's mergesort is conventionally written as a two-phase algorithm; expressing it as `hylo` is educational.

## Key Differences

1. **Fusion**: `hylo` avoids the intermediate Fix-wrapped structure; `ana` + `cata` would allocate and then free it — `hylo` is the allocation-free composition.
2. **Mergesort structure**: Both implementations produce the same O(n log n) mergesort; the `hylo` framing makes the structure/composition explicit.
3. **Streaming**: `hylo` over lazy data structures enables stream fusion — the foundation of Haskell's `stream-fusion` and Rust's zero-allocation iterator chains.
4. **Category theory**: `hylo` = `cata . ana`; fusion is justified by the `hylo` lemma in category theory (a `hylo` decomposition always exists).

## Exercises

1. Implement `sum_range(n)` as `hylo(range_coalg, sum_alg)` — sum all integers from 1 to n without building the list.
2. Verify that mergesort via `hylo` produces the same results as `sort` on 1000 random integers.
3. Measure allocation: compare `ana` + `cata` (materializing the tree) vs. `hylo` (fused) for mergesort on 10,000 elements.
