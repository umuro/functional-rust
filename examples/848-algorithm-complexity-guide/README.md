📖 **[View on hightechmind.io →](https://hightechmind.io/rust/848-algorithm-complexity-guide)**

---

# Algorithm Complexity Guide

## Problem Statement

Understanding time and space complexity is essential for writing software that scales. A function that runs in 1ms for n=100 might take 10 seconds for n=10,000 if it's O(n^2) — and 3 years for n=1,000,000. Engineers need an intuitive grasp of which complexity class their code falls into, and which operations trigger which class. This reference guide concretizes the abstract Big-O notation with real Rust examples: constant-time array access, logarithmic binary search, linear scans, O(n log n) sorting, quadratic nested loops, exponential backtracking. Each example demonstrates not just the code but why it achieves its complexity class.

## Learning Outcomes

- Recognize O(1), O(log n), O(n), O(n log n), O(n^2), O(n^3), O(2^n), O(n!) from code structure
- Understand how recursion depth, loop nesting, and problem halving determine complexity
- Apply Master Theorem to divide-and-conquer recurrences: T(n) = aT(n/b) + f(n)
- Calculate the practical limit for each complexity class: O(n^2) for n=10,000 (~10^8 ops) is borderline
- Recognize amortized complexity: Vec push is O(1) amortized despite occasional O(n) reallocation

## Rust Application

```rust
// O(1) - Constant
pub fn constant_time_example(arr: &[i32]) -> Option<i32> {
    arr.first().copied()  // Index access regardless of arr.len()
}
// O(log n) - Binary search
pub fn logarithmic_example(sorted: &[i32], target: i32) -> bool {
    sorted.binary_search(&target).is_ok()  // Halves problem each step
}
// O(n) - Linear scan
pub fn linear_example(arr: &[i32]) -> i32 {
    arr.iter().sum()  // Visits each element exactly once
}
// O(n^2) - Nested loops
pub fn quadratic_example(arr: &[i32]) -> Vec<(i32, i32)> {
    arr.iter().flat_map(|&x| arr.iter().map(move |&y| (x, y))).collect()
}
```

Each example pairs the implementation with a brief comment explaining why it achieves its complexity class. `first()` is O(1) because Vec/slice has a pointer to the first element. `binary_search` is O(log n) because it halves the search space each step. `sum()` via `iter()` is O(n) because it visits each element once. The nested `flat_map` + `map` creates O(n^2) pairs. The code serves as a reference: when you see these patterns, you know the complexity.

## OCaml Approach

OCaml's complexity guide mirrors Rust's: `List.hd` for O(1), `List.nth` for O(n) (lists are not random-access!), array `.(i)` for O(1), `Array.binary_search` for O(log n). OCaml's lazy `Seq` enables O(1) per-element consumption of infinite sequences. `List.length` is O(n) in OCaml (not cached), unlike Rust's `Vec::len()` which is O(1). This highlights a critical difference: OCaml lists don't cache length; `Array.length` is O(1). Understanding the OCaml stdlib's complexity is essential for writing efficient OCaml.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| `len()` | `Vec::len()` is O(1) | `List.length` is O(n), `Array.length` is O(1) |
| Indexing | `slice[i]` is O(1) | `array.(i)` is O(1), `List.nth` is O(n) |
| Sort | `slice.sort()` is O(n log n) | `List.sort` is O(n log n) |
| HashMap | `O(1)` amortized insert/lookup | `Hashtbl` O(1) average |
| BTreeMap | `O(log n)` insert/lookup | `Map.t` O(log n) |
| Stack overflow | Happens at ~8KB stack depth | TCO prevents for tail calls |

## Exercises

1. Write a benchmark that empirically measures sorting time for n=100,1000,10000,100000 and verify O(n log n) by fitting a curve.
2. Find an example in this codebase where the actual complexity differs from what the code appears to be.
3. Analyze the amortized complexity of `Vec::push`: prove that n pushes cost O(n) total despite occasional O(n) copies.
4. Implement and benchmark a function with O(n^3) complexity and find the practical n where it exceeds 1 second.
5. Explain why `HashMap::get` is O(1) amortized but has O(n) worst case, and when worst case can be triggered.
