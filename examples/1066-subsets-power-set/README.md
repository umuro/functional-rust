📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1066-subsets-power-set)**

---

# 1066-subsets-power-set — Subsets / Power Set
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The power set of a set S is the set of all subsets, including the empty set and S itself. A set with n elements has 2^n subsets. Generating all subsets is needed for exhaustive search, testing all combinations of feature flags, computing all possible teams from a player list, and in model checking for state space exploration.

Two elegant algorithms: backtracking (include/exclude each element) and bitmasking (each bit of an integer represents whether an element is included).

## Learning Outcomes

- Generate all 2^n subsets using backtracking
- Generate subsets using bitmask enumeration over integers 0..2^n
- Understand the bijection between subsets and binary numbers
- Handle subsets with duplicates by sorting and skipping
- Connect to power set semantics in set theory and logic

## Rust Application

`src/lib.rs` implements `subsets_backtrack` by starting with an empty set and at each position either including the current element and recursing, or skipping it. The recursive call always adds the current partial set before diving deeper, ensuring the empty set and all subsets are captured. `subsets_bitmask` enumerates integers from 0 to 2^n-1 and uses bit testing to select elements.

The bitmask approach is elegant and cache-friendly: the loop is simple and accesses the input array sequentially. The backtracking approach is more general and easier to extend (e.g., for filtered subsets).

## OCaml Approach

```ocaml
let subsets lst =
  let n = List.length lst in
  let arr = Array.of_list lst in
  List.init (1 lsl n) (fun mask ->
    List.init n (fun i ->
      if mask land (1 lsl i) <> 0 then [arr.(i)] else []
    ) |> List.concat
  )
```

Or with backtracking:
```ocaml
let rec subsets = function
  | [] -> [[]]
  | x :: rest ->
    let without = subsets rest in
    without @ List.map (fun s -> x :: s) without
```

The recursive OCaml version is clean and idiomatic — divide into subsets with and without the head element.

## Key Differences

1. **Recursive elegance**: OCaml's `subsets` function is 3 lines using `@ List.map`; Rust's backtracking is more verbose but explicit.
2. **Bitmask**: Both languages express `mask & (1 << i) != 0` identically — bitmasking is language-independent.
3. **Memory**: Rust's backtracking collects into `Vec<Vec<i32>>` explicitly; OCaml's recursive version builds lists naturally.
4. **Lazy subsets**: Rust can generate subsets lazily with a bitmasking iterator; OCaml's lazy `Seq` version follows the same pattern.

## Exercises

1. Implement `subsets_of_size(nums: &[i32], k: usize) -> Vec<Vec<i32>>` that generates only subsets of exactly size k.
2. Write `subsets_unique(nums: &mut [i32]) -> Vec<Vec<i32>>` that handles duplicate elements by sorting and skipping duplicates at each level.
3. Implement a lazy subset generator `fn subsets_iter(nums: &[i32]) -> impl Iterator<Item=Vec<i32>>` using bitmask iteration.
