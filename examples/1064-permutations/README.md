📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1064-permutations)**

---

# 1064-permutations — Generate All Permutations

## Problem Statement

Generating all permutations of a sequence is essential for brute-force combinatorial search: testing all orderings of tasks for a scheduling problem, generating all possible moves in game AI, and exhaustive testing of commutative operations. There are n! permutations of n elements.

Two classic algorithms: the swap-based approach (Heap's algorithm variant) and the selection approach using a "used" flags array. Both produce all n! permutations but in different orders.

## Learning Outcomes

- Implement permutation generation via swap-based backtracking
- Implement permutation generation via selection with a used-flags array
- Compare the two approaches for their properties (lexicographic order, allocation patterns)
- Handle duplicates by sorting and skipping repeated elements
- Generate permutations lazily using iterators

## Rust Application

`src/lib.rs` implements `permutations_swap` using in-place swapping: fix a position, swap each remaining element into it, recurse, undo. `permutations_flags` uses a boolean `used` array and a `current` buffer — cleaner but requires an extra allocation per element. Both collect all permutations into `Vec<Vec<i32>>`.

The swap-based approach generates permutations in a non-lexicographic order (Heap's algorithm order); the flags approach generates them in lexicographic order if the input is sorted.

## OCaml Approach

```ocaml
let permutations lst =
  let n = List.length lst in
  let arr = Array.of_list lst in
  let results = ref [] in
  let used = Array.make n false in
  let current = Array.make n 0 in
  let rec build len =
    if len = n then results := Array.to_list current :: !results
    else
      for i = 0 to n - 1 do
        if not used.(i) then begin
          current.(len) <- arr.(i);
          used.(i) <- true;
          build (len + 1);
          used.(i) <- false
        end
      done
  in
  build 0;
  !results
```

Structurally identical. OCaml's mutable arrays and refs replace Rust's explicit `&mut` parameters.

## Key Differences

1. **Swap vs copy**: Rust's swap-based version modifies the input array in place; OCaml's selection approach builds into a separate `current` array.
2. **Lexicographic order**: The selection/flags approach produces permutations in lexicographic order when the input is sorted; swap-based does not.
3. **Memory allocation**: Both collect all n! permutations, allocating O(n × n!) memory total. Lazy permutation generation (via iterators) avoids this.
4. **`itertools::permutations`**: The `itertools` crate provides `iter.permutations(k)` for lazy k-permutations in Rust; OCaml's `Base.List` has no direct equivalent.

## Exercises

1. Implement `permutations_lazy(nums: Vec<i32>) -> impl Iterator<Item=Vec<i32>>` using Heap's algorithm as a lazy iterator.
2. Write `permutations_unique(nums: &mut [i32]) -> Vec<Vec<i32>>` that handles duplicate elements by sorting and skipping repeated swaps.
3. Implement `nth_permutation(nums: &[i32], n: usize) -> Vec<i32>` that generates only the nth permutation in lexicographic order using factoradic number representation.
