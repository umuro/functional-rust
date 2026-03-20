# Generating All Subsets of a List Recursively

**Source:** https://v2.ocaml.org/learn/tutorials/99problems.html

**Difficulty:** Intermediate

## Problem Statement

The power set of a set with n elements contains 2ⁿ subsets. Generating all subsets is fundamental to combinatorial algorithms: exhaustive search, constraint satisfaction, feature selection in machine learning, and generating test cases. The recursive insight is elegant — for each element, either include it or exclude it, then recurse on the rest. This divide-and-conquer structure maps directly onto functional recursion.

## Learning Outcomes

- Understand the recursive structure of subset generation (include/exclude branching)
- Learn how to accumulate results across recursive branches in Rust
- See how the exponential output size (2ⁿ) relates to the linear depth of recursion
- Practice returning `Vec<Vec<T>>` from recursive functions correctly

## Rust Application

In Rust, subset generation returns a `Vec<Vec<T>>`. The recursive function takes a slice `&[T]` and splits it into the first element and the rest. It recurses on the rest to get all subsets without the first element, then clones each of those subsets and prepends the first element to get subsets containing it. The two groups are concatenated and returned. Since `T: Clone` is required to copy elements into new subsets, the type bound must appear in the function signature.

## OCaml Approach

OCaml's idiomatic solution uses pattern matching and list comprehension style:
```ocaml
let rec subsets = function
  | [] -> [[]]
  | x :: rest ->
    let without = subsets rest in
    let with_x = List.map (fun s -> x :: s) without in
    without @ with_x
```
OCaml lists are persistent and cheap to prepend, so `x :: s` costs O(1). The garbage collector manages all the shared list structure automatically.

## Key Differences

1. **Cloning cost**: OCaml's persistent lists share tails, so `x :: s` is O(1); Rust must `clone()` each subset to add an element, making it O(k) per subset.
2. **Pattern matching**: OCaml's `| [] -> ... | x :: rest ->` is the canonical idiom; Rust uses `if let Some((first, rest)) = slice.split_first()`.
3. **Output type**: OCaml returns `'a list list`; Rust returns `Vec<Vec<T>>` with explicit `Clone` bound on `T`.
4. **Accumulator style**: Rust often uses `extend` to combine result vectors; OCaml uses `@` (list append) naturally.

## Exercises

1. Generate all subsets of `[1, 2, 3, 4]` and verify there are exactly 16 (2⁴) subsets including the empty set.
2. Modify the function to generate only subsets of a fixed size k (combinations C(n, k)).
3. Add a filter predicate parameter so only subsets satisfying a condition (e.g., sum > threshold) are returned.
