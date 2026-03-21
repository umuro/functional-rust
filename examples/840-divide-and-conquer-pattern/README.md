📖 **[View on hightechmind.io →](https://hightechmind.io/rust/840-divide-and-conquer-pattern)**

---

# Divide and Conquer Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Divide and conquer is an algorithm design paradigm that decomposes a problem into smaller independent subproblems, solves them recursively, and combines solutions. It underlies merge sort, quicksort, binary search, FFT, Strassen matrix multiplication, and closest pair of points. The recurrence T(n) = aT(n/b) + f(n) is analyzed by the Master Theorem. Understanding divide and conquer as a pattern — not just specific algorithms — enables recognizing and applying it to new problems. Many problems that appear to require O(n^2) naive solutions admit O(n log n) or better divide-and-conquer solutions.

## Learning Outcomes

- Identify the three components: divide (split problem), conquer (recursive solve), combine (merge solutions)
- Apply Master Theorem to analyze recurrences: T(n) = aT(n/b) + O(n^k)
- Implement generic divide-and-conquer scaffolding with configurable split, solve, and merge operations
- Recognize when divide-and-conquer helps: independent subproblems with efficient merge
- Compare with dynamic programming: D&C has independent subproblems; DP has overlapping ones

## Rust Application

```rust
pub fn divide_and_conquer<T, R, F, G, H>(
    items: &[T],
    base_case: &F,
    divide: &G,
    combine: &H,
) -> R
where
    F: Fn(&[T]) -> R,
    G: Fn(&[T]) -> (&[T], &[T]),
    H: Fn(R, R) -> R,
{
    if items.len() <= 1 { return base_case(items); }
    let (left, right) = divide(items);
    let l = divide_and_conquer(left, base_case, divide, combine);
    let r = divide_and_conquer(right, base_case, divide, combine);
    combine(l, r)
}
```

The generic higher-order function parameterizes over element type `T` and result type `R`. The three function parameters `base_case`, `divide`, and `combine` capture the algorithm's three phases. Rust's trait bounds `F: Fn(&[T]) -> R` enforce the correct function signatures. The references to function closures avoid copying them at each recursive call. This pattern enables implementing merge sort, sum queries, and min-max range queries by supplying different functions.

## OCaml Approach

OCaml's higher-order functions make the D&C pattern elegant: `let rec dc base_case divide combine items = if small items then base_case items else let (l, r) = divide items in combine (dc base_case divide combine l) (dc base_case divide combine r)`. The `divide_and_conquer` function is naturally polymorphic. OCaml's `Array.sub` for splitting and `@` or `Array.append` for combining arrays. The parallel version uses `Domain.spawn` (OCaml 5.0) to run both halves concurrently.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Generic params | `<T, R, F, G, H>` with trait bounds | Polymorphic `'a -> 'b` |
| Function passing | References to closures | First-class functions |
| Array splitting | Slice references `&[T]` | `Array.sub` copies |
| Parallelism | `rayon::join` | `Domain.spawn` (OCaml 5.0) |
| Tail recursion | Not applicable (tree recursion) | Not TCO here |
| Pattern recognition | Master Theorem | Same analysis |

## Exercises

1. Implement merge sort using the `divide_and_conquer` generic function with appropriate `divide` and `combine` closures.
2. Implement binary search as a divide-and-conquer: `divide` splits at midpoint, `combine` picks the recursive result.
3. Apply divide and conquer to count inversions in an array in O(n log n) by modifying merge sort.
4. Parallelize the divide step using Rayon's `join` and measure speedup on merge sort for n = 10^7.
5. Implement the divide-and-conquer closest pair using the generic scaffolding above with appropriate functions.
